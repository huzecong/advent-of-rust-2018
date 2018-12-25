import copy
import re
from typing import NamedTuple, List, Tuple


class Group:
    units: int
    hp: int
    immune_to: List[str]
    weak_to: List[str]
    damage: int
    atk_type: str
    initiative: int

    def __init__(self, **kwargs):
        for k, v in kwargs.items():
            setattr(self, k, v)

    def _replace(self, **kwargs) -> 'Group':
        obj = copy.deepcopy(self)
        for k, v in kwargs.items():
            setattr(obj, k, v)
        return obj


def parse(s: str) -> Group:
    match = re.search(r"(\d+) units each with (\d+) hit points "
                      r"(\(.*?\) )?"
                      r"with an attack that does (\d+) ([a-z]+) damage at initiative (\d+)", s.strip())
    # (immune to ([a-z, ]+))?(;)?(weak to ([a-z, ]+))?
    conditions = match.group(3)
    if conditions is not None:
        immune = re.search(r"immune to ([a-z, ]+)[;)]", conditions)
        immune = immune.group(1).split(', ') if immune is not None else []
        weak = re.search(r"weak to ([a-z, ]+)[;)]", conditions)
        weak = weak.group(1).split(', ') if weak is not None else []
    else:
        immune = weak = []
    return Group(units=int(match.group(1)), hp=int(match.group(2)),
                 immune_to=immune, weak_to=weak,
                 damage=int(match.group(4)), atk_type=match.group(5), initiative=int(match.group(6)))


def compute_damage(attack: Group, defend: Group) -> int:
    if attack.atk_type in defend.immune_to:
        return 0
    damage = attack.damage * attack.units
    if attack.atk_type in defend.weak_to:
        damage *= 2
    return damage


def get_effective_power(group: Group) -> int:
    return group.units * group.damage


def combat(immune: List[Group], infection: List[Group], verbose=False) -> Tuple[bool, int]:
    """
    :return: (Whether immune system wins, total units left)
    """
    immune = copy.deepcopy(immune)
    infection = copy.deepcopy(infection)

    teams = [immune, infection]
    team_names = {id(immune): "Immune System", id(infection): "Infection"}
    current_team = {id(t): t for t in teams}
    defending_team_id = {id(immune): id(infection), id(infection): id(immune)}
    defending_team = {id(immune): infection, id(infection): immune}

    def sum_units():
        return sum(sum(max(0, group.units) for group in t) for t in teams)

    prev_sum_units = sum_units()

    while True:
        if verbose:
            for t in teams:
                name = team_names[id(t)]
                print(f"{name}:")
                for idx, group in enumerate(t):
                    if group.units > 0:
                        print(f"Group {idx + 1} contains {group.units} units")
            print()

        # Target selection
        target_indices = {id(t): [-1] * len(t) for t in teams}
        groups = [[(idx, id(t)) for idx in range(len(t)) if t[idx].units > 0] for t in teams]
        groups = groups[0] + groups[1]

        def _find_target_order_key(tup: Tuple[int, int]):
            idx, team_id = tup
            group = current_team[team_id][idx]
            return get_effective_power(group), group.initiative

        order = sorted(groups, key=_find_target_order_key, reverse=True)
        chosen_target = {id(t): [False] * len(defending_team[id(t)]) for t in teams}

        for idx, team_id in order:
            group = current_team[team_id][idx]
            defending = defending_team[team_id]
            damage = [compute_damage(group, target) if not (chosen_target[team_id][t_idx] or target.units <= 0) else -1
                      for t_idx, target in enumerate(defending)]
            if any(x > 0 for x in damage):
                target_idx = max(range(len(defending)),
                                 key=lambda i: (damage[i], defending[i].damage * defending[i].units,
                                                defending[i].initiative))
                target_indices[team_id][idx] = target_idx
                chosen_target[team_id][target_idx] = True
                # print(f"{team_names[team_id]} group {idx + 1} would deal "
                #       f"defending group {target_idx + 1} {damage[target_idx]} damage")
        # print()

        # Attack
        def _attack_order_key(tup: Tuple[int, int]):
            idx, team_id = tup
            group = current_team[team_id][idx]
            return group.initiative

        order = sorted(groups, key=_attack_order_key, reverse=True)
        killed_target = {id(t): [False] * len(t) for t in teams}
        for idx, team_id in order:
            group = current_team[team_id][idx]
            if group.units <= 0:
                continue
            target_idx = target_indices[team_id][idx]
            if target_idx == -1:
                continue
            target = defending_team[team_id][target_idx]
            damage = compute_damage(group, target)
            kills = damage // target.hp
            target.units -= kills
            if verbose:
                print(f"{team_names[team_id]} group {idx + 1} attacks "
                      f"defending group {target_idx + 1}, killing {kills} units")
            if target.units <= kills:
                killed_target[defending_team_id[team_id]][target_idx] = True
        if verbose:
            print()

        for t in teams:
            if all(group.units <= 0 for group in t):
                sum_units = sum(max(group.units, 0) for group in defending_team[id(t)])
                immune_win = not (id(t) == id(immune))
                return immune_win, sum_units

        cur_sum_units = sum_units()
        if cur_sum_units == prev_sum_units:
            return False, cur_sum_units  # will loop forever
        prev_sum_units = cur_sum_units


def main():
    with open('input.txt', 'r') as f:
        lines = f.read()
    immune, infection = [part.strip().split('\n') for part in lines.split('\n\n')[:2]]
    immune = [parse(line) for line in immune[1:]]
    infection = [parse(line) for line in infection[1:]]

    # Part 1
    _, sum_units = combat(immune, infection)
    print(sum_units)

    # Part 2
    def check(val: int, verbose=False) -> Tuple[bool, int]:
        immune_boost = [group._replace(damage=group.damage + val) for group in immune]
        return combat(immune_boost, infection, verbose=verbose)

    l, r = 0, 10000000
    while l < r:
        mid = (l + r) // 2
        immune_win, sum_units = check(mid)
        if immune_win:
            r = mid
        else:
            l = mid + 1
        # print(l, r)
    boost = l
    print(boost)
    _, sum_units = check(boost)
    print(sum_units)


if __name__ == '__main__':
    main()

from pydracula import Lang, get_meaningful_line_indices

indices = get_meaningful_line_indices(
    Lang.C,
    """
            int xyz() {
                auto x = 10;
            }
            """,
)
assert len(indicies) == 3

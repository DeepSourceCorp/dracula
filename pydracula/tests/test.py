from pydracula import Lang, get_meaningful_line_indicies

indicies = get_meaningful_line_indicies(
    Lang.C,
    """
            int xyz() {
                auto x = 10;
            }
            """,
)
assert len(indicies) == 3

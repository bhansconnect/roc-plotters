interface Config
    exposes [ Coord, Config ]
    imports []

# This should probably be a tag, but I didn't want to mess with it too much rn.
Coord :
    {
        x : I32,
        y : I32,
    }
# TODO: Make this a `List` and support it in the platform (with `Vec`?)
Config :
    {
        outputFilePath : Str,
        title : Str,
        subtitle : Str,
        width : U32,
        height : U32,
        # Does this work or do you need more dimensions?
        points1 : List (Coord),
        points2 : List (Coord),
    }

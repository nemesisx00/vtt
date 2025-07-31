use diesel::table;

table!
{
	imageAssets (id)
	{
		id -> Integer,
		height -> BigInt,
		path -> Text,
		width -> BigInt,
	}
}

table!
{
	messages (id)
	{
		id -> Integer,
		text -> Text,
		timestamp -> Timestamp,
		userId -> Nullable<Integer>,
	}
}

table!
{
	scenes2d (id)
	{
		id -> Integer,
		name -> Text,
		backgroundId -> Integer,
	}
}

table!
{
	users (id)
	{
		id -> Integer,
		label -> Nullable<Text>,
		name -> Text,
	}
}

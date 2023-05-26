import std;

struct Item
{
	string code;
	string name;
	string quantity;
	string[] tags;
	string description;
	string image;
}

void main(string[] args)
{
	Item[] db;
	foreach (file; args[1 .. $])
		db ~= parseItems(file);
	writeln("loaded ", db.length, " items");
	while (auto line = readln())
	{
		foreach (i, item; search(db, line))
		{
			if (i > 10)
				break;
			writeln("#", i+1, ": ", item.name);
		}
	}
}

Item[] parseItems(string file)
{
	auto json = parseJSON(readText(file));
	return json.array.map!(j => Item(
		j["code"].str,
		j["name"].str,
		j["q"].str,
		j["tags"].array.map!(v => v.str).array,
		j["desc"].str,
		j["image"].str
	)).array;
}

Item[] search(Item[] items, string q)
{
	return items[0 .. 1];
}

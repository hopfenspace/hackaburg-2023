import std;

string translateTag(string tag)
{
	switch (tag)
	{
	case "fruits": return "fruits";
	case "eggs": return "eggs";
	case "vegetables": return "vegetables";
	case "bread": return "breads";
	case "flowers": return "flowers";
	case "en:canned-fruits": return "canned-fruits";
	case "en:canned-vegetables": return "canned-vegetables";
	case "en:cheeses": return "cheeses";
	case "en:breads": return "breads";
	case "en:candies": return "candies";
	case "en:eggs": return "eggs";
	case "en:fruits": return "fruits";
	case "en:jams": return "jams";
	case "en:spreads": return "spreads";
	case "en:dairies": return "dairies";
	case "en:chocolates": return "chocolates";
	case "en:milks": return "milks";
	case "en:vegetables": return "vegetables";
	case "en:beverages": return "beverages";
	case "en:snacks": return "snacks";
	case "en:meals": return "meals";
	case "en:dietary-supplements": return "dietary-supplements";
	case "en:condiments": return "condiments";
	case "en:flours": return "flours";
	case "en:sweeteners": return "sweeteners";
	case "en:sugars": return "sugars";
	case "en:pastas": return "pastas";
	case "en:baby-foods": return "baby-foods";
	case "en:plant-based-foods-and-beverages": return "plant-based-foods-and-beverages";
	case "en:desserts": return "desserts";
	case "en:sandwiches": return "sandwiches";
	case "en:meats": return "meats";
	case "en:seafood": return "seafood";
	case "en:broths": return "broths";
	default: return "";
	}
}

void main(string[] args)
{
	foreach (file; args[1 .. $])
	{
		writeln(file);
		work(file);
	}

	writeln("Done ", args.length - 1, " files");
}

void work(string file)
{
	struct Data
	{
		string code;
		string product_name_de;
		string quantity;
		string[] categories_tags;
		string ingredients_text_de;
		string image;
	}

	Data[] ret;
	string[] headerLine;

	void put(string[] line)
	{
		assert(line[0].all!isDigit, line[0]);
		ret ~= Data(
			line[headerLine.countUntil("code")],
			line[headerLine.countUntil("product_name_de")],
			line[headerLine.countUntil("quantity")],
			line[headerLine.countUntil("categories_tags")].split(",").map!translateTag.filter!(a => a.length > 0).array,
			line[headerLine.countUntil("ingredients_text_de")],
		);
		auto productNameEn = headerLine.countUntil("product_name_en");
		if (productNameEn != -1 && !ret[$ - 1].product_name_de.length)
			ret[$ - 1].product_name_de = line[productNameEn];
		auto img = headerLine.countUntil("image");
		if (img != -1)
			ret[$ - 1].image = line[img];
		if (ret[$ - 1].categories_tags.length == 0 && line[headerLine.countUntil("categories_tags")].length)
			writeln("No tags for ", line[headerLine.countUntil("categories_tags")]);
	}

	bool inString, acceptQuote;
	string[] line = [""];
	foreach (char c; readText(file))
	{
		if (inString)
		{
			if (c == '"')
			{
				inString = false;
				acceptQuote = true;
				continue;
			}
			else
				line[$ - 1] ~= c;
		}
		else
		{
			if (c == '\n')
			{
				if (!headerLine.length)
					headerLine = line;
				else
					put(line);
				line = [""];
			}
			else if (c == '\t')
			{
				acceptQuote = false;
				line ~= "";
			}
			else if (c == '"')
			{
				if (acceptQuote)
				{
					line[$ - 1] ~= '"';
					inString = true;
				}
				else if (line[$ - 1].length)
				{
					line[$ - 1] ~= '"';
				}
				else
					inString = true;
			}
			else
				line[$ - 1] ~= c;
			acceptQuote = false;
		}
	}

	JSONValue[] json;
	foreach (item; ret)
	{
		auto j = [
			"name": JSONValue(item.product_name_de),
			"main_category": JSONValue(item.categories_tags.length ? item.categories_tags[$ - 1] : "all"),
		];
		if (item.image.length)
			j["image"] = JSONValue(item.image);
		if (item.ingredients_text_de.length)
			j["description"] = JSONValue(item.ingredients_text_de);
		if (item.code.length)
			j["ean_code"] = JSONValue(item.code);
		if (item.quantity.length)
			j["quantity"] = JSONValue(item.quantity);
		json ~= JSONValue(j);
	}

	import std.file : write;
	write(file ~ ".json", JSONValue(json).toString);
}
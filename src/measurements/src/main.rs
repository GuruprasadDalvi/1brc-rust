use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use rand::seq::SliceRandom; // Import the necessary trait
use rand::{thread_rng, Rng};
fn main() {
    let cities = [
        "mumbai",
        "delhi",
        "bangalore",
        "hyderabad",
        "ahmedabad",
        "chennai",
        "kolkata",
        "surat",
        "pune",
        "jaipur",
        "lucknow",
        "kanpur",
        "nagpur",
        "visakhapatnam",
        "indore",
        "thane",
        "bhopal",
        "patna",
        "vadodara",
        "ghaziabad",
        "ludhiana",
        "coimbatore",
        "agra",
        "madurai",
        "nashik",
        "faridabad",
        "meerut",
        "rajkot",
        "kalyan-dombivali",
        "vasai-virar",
        "varanasi",
        "srinagar",
        "aurangabad",
        "dhanbad",
        "amritsar",
        "navi mumbai",
        "allahabad",
        "howrah",
        "ranchi",
        "gwalior",
        "jabalpur",
        "jodhpur",
        "raipur",
        "kota",
        "guwahati",
        "chandigarh",
        "solapur",
        "hubli-dharwad",
        "bareilly",
        "moradabad",
        "mysore",
        "gurgaon",
        "aligarh",
        "jalandhar",
        "tiruchirappalli",
        "bhubaneswar",
        "salem",
        "mira-bhayandar",
        "warangal",
        "thiruvananthapuram",
        "guntur",
        "bhiwandi",
        "saharanpur",
        "gorakhpur",
        "bikaner",
        "amravati",
        "noida",
        "jamshedpur",
        "bhilai",
        "cuttack",
        "kochi",
        "uzhno-sakhalinsk",
        "khabarovsk",
        "vancouver",
        "victoria",
        "abbotsford",
        "kelowna",
        "nanaimo",
        "kamloops",
        "prince george",
        "chilliwack",
        "nyc",
        "la",
        "chicago",
        "houston",
        "phoenix",
        "philadelphia",
        "san antonio",
        "san diego",
        "dallas",
        "san jose",
        "austin",
        "jacksonville",
        "san francisco",
        "columbus",
        "fort worth",
        "indianapolis",
        "charlotte",
        "seattle",
        "denver",
        "washington",
        "boston",
        "el paso",
        "detroit",
        "nashville",
        "memphis",
        "portland",
        "oklahoma city",
        "las vegas",
        "louisville",
        "baltimore",
        "milwaukee",
        "albuquerque",
        "tucson",
        "fresno",
        "sacramento",
        "mesa",
        "kansas city",
        "atlanta",
        "long beach",
        "colorado springs",
        "raleigh",
        "miami",
        "virginia beach",
        "omaha",
        "oakland",
        "minneapolis",
        "tulsa",
        "arlington",
        "new orleans",
        "wichita",
        "cleveland",
        "tampa",
        "bakersfield",
        "aurora",
        "honolulu",
        "anaheim",
        "santa ana",
        "corpus christi",
        "riverside",
        "lexington",
        "stockton",
        "pittsburgh",
        "st. paul",
        "anchorage",
        "cincinnati",
        "henderson",
        "greensboro",
        "plano",
        "newark",
        "toledo",
        "lincoln",
        "orlando",
        "chula vista",
        "irvine",
        "fort wayne",
        "jersey city",
        "durham",
        "st. petersburg",
        "laredo",
        "buffalo",
        "madison",
        "lubbock",
        "chandler",
        "scottsdale",
        "glendale",
        "reno",
        "norfolk",
        "winston-salem",
        "north las vegas",
        "irving",
        "chesapeake",
        "gilbert",
        "hialeah",
        "garland",
        "fremont",
        "baton rouge",
        "richmond",
        "boise",
        "san bernardino",
        "spokane",
        "birmingham",
        "modesto",
        "des moines",
        "rochester",
        "tacoma",
        "fontana",
        "oceanside",
        "mobile",
        "vancouver",
        "victoria",
        "abbotsford",
        "kelowna",
        "nanaimo",
        "kamloops",
        "dublin",
        "cork",
        "limerick",
        "galway",
        "waterford",
        "drogheda",
        "dundalk",
        "derry",
        "belfast",
        "lisburn",
        "newtownabbey",
        "bangor",
        "craigavon",
        "castlereagh",
        "ballymena",
        "newtownards",
        "coleraine",
        "omagh",
        "tokyo",
        "yokohama",
        "osaka",
        "nagoya",
        "sapporo",
        "kobe",
        "kyoto",
        "fukuoka",
        "kawasaki",
        "saitama",
        "hiroshima",
        "sendai",
        "kitakyushu",
        "chiba",
        "sakai",
        "niigata",
        "hamamatsu",
        "sagamihara",
        "okayama",
        "kumamoto",
        "kanazawa",
        "nara",
        "kagoshima",
        "kochi",
        "akita",
        "aomori",
        "morioka",
        "fukushima",
        "yamagata",
        "nagano",
        "niigata",
        "shizuoka",
        "hamamatsu",
        "gifu",
        "shizuoka",
        "dubai",
        "abu dhabi",
        "sharjah",
        "al ain",
        "ajman",
        "ras al khaimah",
        "um al quwain",
        "khor fakkan",
        "kalba",
        "dibba al-fujairah",
        "madrid",
        "barcelona",
        "valencia",
        "seville",
        "zaragoza",
        "málaga",
        "murcia",
        "palma",
        "las palmas",
        "bilbao",
        "alicante",
        "córdoba",
        "valladolid",
        "vigo",
        "gijón",
        "hospitalet",
        "la coruña",
        "granada",
        "vitoria",
        "elche",
        "oviedo",
    ];

    let args: Vec<String> = env::args().collect();
    let mut count_str = args.get(1);
    let filename_str = args.get(2);

    let mut count = 1_000_000;
    if count_str.is_some() {
        count = count_str.unwrap().parse().unwrap();
    }

    let mut filename = "weather_stations.csv";
    if filename_str.is_some() {
        filename = filename_str.unwrap();
    }
    let mut data = String::from("");
    let mut buf =  BufWriter::new(File::create(filename).unwrap());
    for _ in 0..count{
        let mut rng = thread_rng();
        let c = cities.choose(&mut rng);
        let number: f64 = rng.gen_range(-10.0..35.0);
        writeln!(buf,"{};{}",c.clone().unwrap(),number.to_string());
    }

}
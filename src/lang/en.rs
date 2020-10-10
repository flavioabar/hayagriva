pub const NEVER_CAPITALIZE: [&str; 46] = [
    "a",
    "above",
    "across",
    "against",
    "among",
    "an",
    "and",
    "around",
    "as",
    "at",
    "behind",
    "below",
    "beneath",
    "beside",
    "between",
    "but",
    "by",
    "during",
    "for",
    "from",
    "front",
    "in",
    "inside",
    "into",
    "m",
    "n",
    "near",
    "nor",
    "of",
    "on",
    "onto",
    "or",
    "over",
    "s",
    "since",
    "so",
    "t",
    "the",
    "to",
    "toward",
    "under",
    "underneath",
    "until",
    "with",
    "within",
    "yet",
];

const ALWAYS_CAPITALIZE: [&str; 395] = [
    "ababa",
    "abidjan",
    "addis",
    "afghanistan",
    "africa",
    "african",
    "ahmedabad",
    "aires",
    "albania",
    "alexandria",
    "algeria",
    "algiers",
    "america",
    "amsterdam",
    "andorra",
    "angeles",
    "angola",
    "antigua",
    "april",
    "arab",
    "arabia",
    "argentina",
    "armenia",
    "athens",
    "atlanta",
    "august",
    "australia",
    "austria",
    "azerbaijan",
    "baghdad",
    "bahamas",
    "bahrain",
    "bangalore",
    "bangkok",
    "bangladesh",
    "barbados",
    "barbuda",
    "barcelona",
    "beijing",
    "belarus",
    "belgium",
    "belize",
    "belo",
    "benin",
    "berlin",
    "bernardino",
    "bhutan",
    "birmingham",
    "bissau",
    "bogotá",
    "bolivia",
    "bonn",
    "bosnia",
    "boston",
    "botswana",
    "brasília",
    "brazil",
    "brunei",
    "brussels",
    "bucharest",
    "budapest",
    "buenos",
    "bulgaria",
    "burkina faso",
    "burma",
    "burundi",
    "cabo verde",
    "cairo",
    "cambodia",
    "cameroon",
    "canada",
    "casablanca",
    "chad",
    "chengdu",
    "chennai",
    "chi",
    "chicago",
    "chile",
    "china",
    "chongqing",
    "city",
    "cologne",
    "colombia",
    "comoros",
    "congo",
    "congo-brazzaville",
    "copenhagen",
    "costa rica",
    "croatia",
    "cuba",
    "cyprus",
    "czech",
    "czechia",
    "côte",
    "d'ivoire",
    "dakar",
    "dalian",
    "dallas",
    "dar",
    "december",
    "delhi",
    "denmark",
    "detroit",
    "dhaka",
    "djibouti",
    "dominica",
    "dominican",
    "dongguan",
    "douala",
    "dr",
    "dublin",
    "durban",
    "ecuador",
    "egypt",
    "el",
    "emirates",
    "equatorial",
    "eritrea",
    "estonia",
    "eswatini",
    "ethiopia",
    "february",
    "fiji",
    "finland",
    "fortaleza",
    "foshan",
    "france",
    "francisco",
    "frankfurt",
    "friday",
    "fukuoka",
    "gabon",
    "gambia",
    "georgia",
    "germany",
    "ghana",
    "greece",
    "grenada",
    "grenadines",
    "guadalajara",
    "guangzhou",
    "guatemala",
    "guinea",
    "guinea",
    "guinea-bissau",
    "guyana",
    "hague",
    "haiti",
    "hamburg",
    "hangzhou",
    "hanoi",
    "harbin",
    "herzegovina",
    "ho",
    "holy",
    "honduras",
    "hong",
    "horizonte",
    "houston",
    "hungary",
    "hyderabad",
    "i",
    "ibadan",
    "iceland",
    "india",
    "indonesia",
    "iran",
    "iraq",
    "ireland",
    "islands",
    "israel",
    "istanbul",
    "italy",
    "jakarta",
    "jamaica",
    "janeiro",
    "january",
    "japan",
    "jinan",
    "jinjiang",
    "johannesburg",
    "jordan",
    "july",
    "june",
    "kano",
    "karachi",
    "kazakhstan",
    "kenya",
    "khartoum",
    "kiev",
    "kingdom",
    "kinshasa",
    "kiribati",
    "kitts",
    "kobe",
    "kolkata",
    "kong",
    "korea",
    "kuala",
    "kuwait",
    "kyoto",
    "kyrgyzstan",
    "lagos",
    "lahore",
    "lanka",
    "laos",
    "latvia",
    "lebanon",
    "leone",
    "lesotho",
    "liberia",
    "libya",
    "liechtenstein",
    "lima",
    "lisbon",
    "lithuania",
    "london",
    "los",
    "luanda",
    "lucia",
    "lumpur",
    "luxembourg",
    "lyon",
    "macedonia",
    "madagascar",
    "madrid",
    "malawi",
    "malaysia",
    "maldives",
    "mali",
    "malta",
    "manchester",
    "manila",
    "march",
    "marino",
    "marseille",
    "mauritania",
    "mauritius",
    "may",
    "medellín",
    "mexico",
    "miami",
    "micronesia",
    "milan",
    "minh",
    "mogadishu",
    "moldova",
    "monaco",
    "monday",
    "mongolia",
    "montenegro",
    "monterrey",
    "montreal",
    "morocco",
    "moscow",
    "mozambique",
    "mr",
    "mrs",
    "mumbai",
    "munich",
    "myanmar",
    "nagoya",
    "nairobi",
    "namibia",
    "nanjing",
    "naples",
    "nauru",
    "nepal",
    "netherlands",
    "nevis",
    "nicaragua",
    "niger",
    "nigeria",
    "norway",
    "november",
    "oakland",
    "october",
    "oman",
    "osaka",
    "ouagadougou",
    "pakistan",
    "palau",
    "palestine",
    "panama",
    "papua",
    "paraguay",
    "paris",
    "paulo",
    "peru",
    "petersburg",
    "philadelphia",
    "philippines",
    "phoenix",
    "poland",
    "porto alegre",
    "portugal",
    "prague",
    "pune",
    "qatar",
    "qingdao",
    "quanzhou",
    "recife",
    "republic",
    "rio",
    "riverside",
    "riyadh",
    "romania",
    "rome",
    "rotterdam",
    "ruhr",
    "russia",
    "rwanda",
    "saint",
    "salaam",
    "salvador",
    "samoa",
    "san",
    "santiago",
    "sao",
    "saturday",
    "saudi",
    "seattle",
    "senegal",
    "seoul",
    "september",
    "serbia",
    "seychelles",
    "shanghai",
    "shenyang",
    "shenzhen",
    "shishi",
    "sierra",
    "singapore",
    "slovakia",
    "slovenia",
    "sofia",
    "solomon",
    "somalia",
    "spain",
    "sri",
    "states",
    "stockholm",
    "sudan",
    "sunday",
    "surat",
    "suriname",
    "suzhou",
    "swaziland",
    "sweden",
    "switzerland",
    "syria",
    "são",
    "tajikistan",
    "tanzania",
    "tehran",
    "thailand",
    "thursday",
    "tianjin",
    "timor-leste",
    "tobago",
    "togo",
    "tokyo",
    "tome",
    "tonga",
    "toronto",
    "trinidad",
    "tuesday",
    "tunisia",
    "turin",
    "turkey",
    "turkmenistan",
    "tuvalu",
    "uganda",
    "uk",
    "ukraine",
    "united",
    "uruguay",
    "us",
    "usa",
    "uzbekistan",
    "vanuatu",
    "venezuela",
    "vienna",
    "vietnam",
    "vincent",
    "warsaw",
    "washington",
    "wednesday",
    "wuhan",
    "xi'an",
    "yangon",
    "yemen",
    "york",
    "zambia",
    "zimbabwe",
];

use phf;
use std::ascii::AsciiExt;
use super::namepart::NamePart;

static TWO_CHAR_TITLES: [&'static str; 4] = [
    "mr",
    "ms",
    "sr",
    "dr",
];

static TITLE_PARTS: phf::Set<&'static str> = phf_set! {
    "Aunt",
    "Auntie",
    "Attaché",
    "Dame",
    "Marchioness",
    "Marquess",
    "Marquis",
    "Marquise",
    "King",
    "King'S",
    "Queen",
    "Queen'S",
    "1lt",
    "1st",
    "1sgt",
    "1stlt",
    "1stsgt",
    "2lt",
    "2nd",
    "2ndlt",
    "A1c",
    "Abbess",
    "Abbot",
    "Academic",
    "Acolyte",
    "Adept",
    "Adjutant",
    "Adm",
    "Admiral",
    "Advocate",
    "Akhoond",
    "Air",
    "Ald",
    "Alderman",
    "Almoner",
    "Ambassador",
    "Amn",
    "Analytics",
    "Appellate",
    "Apprentice",
    "Arbitrator",
    "Archbishop",
    "Archdeacon",
    "Archdruid",
    "Archduchess",
    "Archduke",
    "Arhat",
    "Assistant",
    "Assoc",
    "Associate",
    "Asst",
    "Attache",
    "Attorney",
    "Ayatollah",
    "Baba",
    "Bailiff",
    "Banner",
    "Bard",
    "Baron",
    "Barrister",
    "Bearer",
    "Bench",
    "Bgen",
    "Bishop",
    "Blessed",
    "Bodhisattva",
    "Brig",
    "Brigadier",
    "Briggen",
    "Brother",
    "Buddha",
    "Burgess",
    "Business",
    "Bwana",
    "Canon",
    "Capt",
    "Captain",
    "Cardinal",
    "Chargé",
    "Catholicos",
    "Ccmsgt",
    "Cdr",
    "Ceo",
    "Cfo",
    "Chair",
    "Chairs",
    "Chancellor",
    "Chaplain",
    "Chief",
    "Chieftain",
    "Civil",
    "Clerk",
    "Cmd",
    "Cmdr",
    "Cmsaf",
    "Cmsgt",
    "Co-Chair",
    "Co-Chairs",
    "Coach",
    "Col",
    "Colonel",
    "Commander",
    "Commander-In-Chief",
    "Commodore",
    "Comptroller",
    "Controller",
    "Corporal",
    "Corporate",
    "Councillor",
    "Count",
    "Countess",
    "Courtier",
    "Cpl",
    "Cpo",
    "Cpt",
    "Credit",
    "Criminal",
    "Csm",
    "Curator",
    "Customs",
    "Cwo",
    "Cwo-2",
    "Cwo-3",
    "Cwo-4",
    "Cwo-5",
    "Cwo2",
    "Cwo3",
    "Cwo4",
    "Cwo5",
    "D'Affaires",
    "Deacon",
    "Delegate",
    "Deputy",
    "Designated",
    "Det",
    "Dir",
    "Director",
    "Discovery",
    "District",
    "Division",
    "Docent",
    "Docket",
    "Doctor",
    "Doyen",
    "Dpty",
    "Druid",
    "Duke",
    "Dutchess",
    "Edmi",
    "Edohen",
    "Effendi",
    "Ekegbian",
    "Elder",
    "Elerunwon",
    "Emperor",
    "Empress",
    "Ens",
    "Envoy",
    "Exec",
    "Executive",
    "Fadm",
    "Family",
    "Father",
    "Federal",
    "Field",
    "Financial",
    "First",
    "Flag",
    "Flying",
    "Flight",
    "Flt",
    "Foreign",
    "Forester",
    "Frau",
    "Friar",
    "Gen",
    "General",
    "Generalissimo",
    "Gentiluomo",
    "Giani",
    "Goodman",
    "Goodwife",
    "Governor",
    "Grand",
    "Group",
    "Guru",
    "Gyani",
    "Gysgt",
    "Hajji",
    "Headman",
    "Her",
    "Herr",
    "Hereditary",
    "High",
    "His",
    "Hon",
    "Honorable",
    "Honourable",
    "Imam",
    "Information",
    "Insp",
    "Intelligence",
    "Intendant",
    "Journeyman",
    "Judge",
    "Judicial",
    "Justice",
    "Junior",
    "Kingdom",
    "Knowledge",
    "Lady",
    "Lama",
    "Lamido",
    "Law",
    "Lcdr",
    "Lcpl",
    "Leader",
    "Lieutenant",
    "Lord",
    "Leut",
    "Lieut",
    "Ltc",
    "Ltcol",
    "Ltg",
    "Ltgen",
    "Ltjg",
    "Madam",
    "Madame",
    "Mag",
    "Mag-Judge",
    "Mag/Judge",
    "Magistrate",
    "Magistrate-Judge",
    "Maharajah",
    "Maharani",
    "Mahdi",
    "Maid",
    "Maj",
    "Majesty",
    "Majgen",
    "Major",
    "Manager",
    "Marcher",
    "Marketing",
    "Marshal",
    "Master",
    "Matriarch",
    "Matron",
    "Mayor",
    "Mcpo",
    "Mcpoc",
    "Mcpon",
    "Member",
    "Metropolitan",
    "Mgr",
    "Mgysgt",
    "Minister",
    "Miss",
    "Misses",
    "Mister",
    "Mme",
    "Monsignor",
    "Most",
    "Mother",
    "Mpco-Cg",
    "Mrs",
    "Msg",
    "Msgr",
    "Msgt",
    "Mufti",
    "Mullah",
    "Municipal",
    "Murshid",
    "Nanny",
    "National",
    "Nurse",
    "Officer",
    "Operating",
    "Pastor",
    "Patriarch",
    "Petty",
    "Pfc",
    "Pharaoh",
    "Pilot",
    "Pir",
    "Po1",
    "Po2",
    "Po3",
    "Police",
    "Political",
    "Pope",
    "Prefect",
    "Prelate",
    "Premier",
    "Pres",
    "Presbyter",
    "President",
    "Presiding",
    "Priest",
    "Priestess",
    "Primate",
    "Prime",
    "Prin",
    "Prince",
    "Princess",
    "Principal",
    "Prior",
    "Private",
    "Pro",
    "Prof",
    "Professor",
    "Provost",
    "Pslc",
    "Pte",
    "Pursuivant",
    "Pv2",
    "Pvt",
    "Rabbi",
    "Radm",
    "Rangatira",
    "Ranger",
    "Rdml",
    "Rear",
    "Rebbe",
    "Registrar",
    "Rep",
    "Representative",
    "Resident",
    "Rev",
    "Revenue",
    "Reverend",
    "Reverand",
    "Revd",
    "Right",
    "Risk",
    "Royal",
    "Saint",
    "Sargent",
    "Sargeant",
    "Saoshyant",
    "Scpo",
    "Secretary",
    "Security",
    "Seigneur",
    "Senator",
    "Senior",
    "Senior-Judge",
    "Sergeant",
    "Servant",
    "Sfc",
    "Sgm",
    "Sgt",
    "Sgtmaj",
    "Sgtmajmc",
    "Shehu",
    "Sheikh",
    "Sheriff",
    "Siddha",
    "Sir",
    "Sister",
    "Sma",
    "Smsgt",
    "Solicitor",
    "Spc",
    "Speaker",
    "Special",
    "Sra",
    "Ssg",
    "Ssgt",
    "Staff",
    "State",
    "States",
    "Strategy",
    "Subaltern",
    "Subedar",
    "Sultan",
    "Sultana",
    "Superior",
    "Supreme",
    "Surgeon",
    "Swordbearer",
    "Sysselmann",
    "Tax",
    "Technical",
    "Timi",
    "Tirthankar",
    "Treasurer",
    "Tsar",
    "Tsarina",
    "Tsgt",
    "Uncle",
    "United",
    "Vadm",
    "Vardapet",
    "Venerable",
    "Verderer",
    "Very",
    "Vicar",
    "Vice",
    "Viscount",
    "Vizier",
    "Warden",
    "Warrant",
    "Wing",
    "Wo-1",
    "Wo-2",
    "Wo-3",
    "Wo-4",
    "Wo-5",
    "Wo1",
    "Wo2",
    "Wo3",
    "Wo4",
    "Wo5",
    "Woodman",
    "And",
    "The",
    "Und",
};

fn might_be_title_part(word: &NamePart) -> bool {
    if word.chars < 3 {
        // Allow any word with 1 or 2 characters as part of a title (but see below)
        true
    } else if word.is_abbreviation() || word.is_initials() {
        true
    } else {
        let namecased = &*word.namecased;
        TITLE_PARTS.contains(namecased)
    }
}

fn might_be_last_title_part(word: &NamePart) -> bool {
    // Don't allow 1 or 2-character words as the whole or final piece of
    // a title, except a set of very-common two-character title abbreviations,
    // because otherwise we are more likely dealing with initials
    if word.chars == 1 {
        false
    }
    else if word.chars == 2 {
        TWO_CHAR_TITLES.iter().any( |title| title.eq_ignore_ascii_case(word.word) )
    }
    else {
        might_be_title_part(word)
    }
}

pub fn is_title(words: &[NamePart]) -> bool {
    match words.last() {
        Some(word) => {
            if !might_be_last_title_part(&word) {
                return false;
            }
        }
        None => {
            return false;
        }
    }

    if words.len() > 1 {
        words[0..words.len()-1].iter().all( |word| might_be_title_part(&word) )
    }
    else {
        true
    }
}

use std::io;
use rustml::bayes_classifier::{Tokenize, DataType, NaiveBayesClassifier};
use std::string::String;
use std::fmt::Display;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
enum Language {
    English,
    Spanish,
    French
}

impl Display for Language {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        match *self {
            Language::English => f.write_str("English"),
            Language::Spanish => f.write_str("Spanish"),
            Language::French => f.write_str("French"),
        }
    }
}

struct Content {
    string: String,
}

impl Tokenize for Content {
    fn tokenize(&self) -> Vec<DataType> {
        let split_by: char = ' ';
        let char_whitelist: [char; 27] = [
            split_by,
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
            'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        ];

        self.string
            .trim()
            .to_lowercase()
            .chars()
                .filter(|character| char_whitelist.contains(&character))
                .map(|character| character.to_string())
                .collect::<DataType>()
            .split(split_by)
            .map(|word| word.to_string()).collect()
    }
}

struct LanguageDetector {
    brain: NaiveBayesClassifier<Language>,
}

impl LanguageDetector {
    pub fn new() -> LanguageDetector {
        LanguageDetector {
            brain: NaiveBayesClassifier::new(),
        }
    }

    pub fn teach(&mut self) {
        // Random English paragraphs from nytimes.com
        self.brain.train(Language::English, Content { string: "From the State Capitol, Gov. Andrew M. Cuomo has established a framework for reopening the state, based on seven concrete, health-related milestones, and he has asked Bill Gates, the restaurateur Danny Meyer, the New York Knicks owner James L. Dolan and dozens of other outside advisers from the upper echelons of New York’s business world to help guide him on how best to restart the economy and, possibly, reimagine public education.".to_string() });
        self.brain.train(Language::English, Content { string: "The factors that made the city the U.S. epicenter of the pandemic — its density, tourism and dependence on mass transit — complicate a return to any semblance of normalcy. The city is still far from meeting the public health metrics necessary to reopen, from available critical-care beds to new hospital admissions for the virus.".to_string() });
        self.brain.train(Language::English, Content { string: "The decision about when to reopen involves a balancing act: The longer New York is shut down, the more the pandemic will abate, reducing the need for testing and contact tracing while allowing officials more time to expand those efforts. But the economic damage to the city and the state will continue to grow.".to_string() });
        self.brain.train(Language::English, Content { string: "Mr. de Blasio works from a mostly empty City Hall in Lower Manhattan with a skeleton staff, speaking to top officials in an endless series of teleconferences and secure video chats. Each day, the mayor has an early call on small decisions — how long to extend a moratorium on alternate-side-of-the-street parking rules, for example — and, later, one or two big discussions about the city’s future.".to_string() });
        self.brain.train(Language::English, Content { string: "But some of the most urgent questions, such as how to handle the normal load of nearly six million daily subway riders, or how 1.1 million school children might actually return to classrooms, have yet to be answered by either the city or the state.".to_string() });
        self.brain.train(Language::English, Content { string: "Schools have been canceled for the rest of the academic year, and the city’s powerful teachers’ union, the United Federation of Teachers, has already said it would expect a system of widely available testing, contact tracing and cleaning to be ready and working before it would support reopening. The union has proposed experimenting with having school in split shifts, morning and afternoon.".to_string() });
        self.brain.train(Language::English, Content { string: "“Until the schools are open, a good subset of working New Yorkers cannot leave their homes,” said Alison Hirsh, a top adviser to Mr. de Blasio and one of several city officials, including deputy mayors Dean Fuleihan and Vicki Been, coordinating the reopening plans.".to_string() });
        self.brain.train(Language::English, Content { string: "“There’s an argument to be made that one of the reasons to keep the schools closed is to continue to force anyone who can work from home to continue to work from home,” Ms. Hirsh said. “That’s one way that you can slow down the reopening and help maintain a flatter curve.”".to_string() });
        self.brain.train(Language::English, Content { string: "Restaurants are an easier problem to solve than schools because the state can rely on occupancy limits, said Jim Malatras, an adviser to the governor on the virus response. Whether restaurants can survive with those limits is an open question.".to_string() });
        self.brain.train(Language::English, Content { string: "City and state officials speak frequently, but Mr. Cuomo appears interested in maintaining an upper hand, according to three people with knowledge of the communications. Most recently, his office did not alert City Hall about Mr. Cuomo’s reopening metrics before he detailed them publicly, though the benchmarks could determine the city’s near future.".to_string() });

        // Random Spanish paragraphs from elmundo.es
        self.brain.train(Language::Spanish, Content { string: "La negociación que hizo posible que Arrimadas acabara apoyando la prórroga del estado de alarma, y cómo la nueva ruta de Cs provoca un tsunami político, con dimisiones y amenazas. Tras una llamada del presidente, a las 14.30 horas del lunes y que duró 30 minutos, el tira y afloja fue obra de Bolaños, el 'padre' de la desescalada, y del negociador de los acuerdos municipales con el PSOE, Carlos Cuadrado. ¿Habrá pacto de presupuestos? El canal de diálogo está abierto".to_string() });
        self.brain.train(Language::Spanish, Content { string: "Los jefes de servicio o responsables de medicina preventiva en una quincena de hospitales de Madrid han enviado una carta a la Comunidad de Madrid en la que piden que no se repartan mascarillas FFP2, para la población general, porque pueden provocar mayores riesgos, y que se elija en su lugar mascarillas quirúrgicas.".to_string() });
        self.brain.train(Language::Spanish, Content { string: "La misiva, a la que ha tenido acceso Efe, fue enviada ayer al consejero de Sanidad de la Comunidad de Madrid, Enrique Ruiz Escudero y al nuevo viceconsejero de Salud Pública, Antonio Zapatero, quien sustituye a Yolanda Fuentes, la directora general que dimitió por no compartir la petición de Madrid de pasar a la fase 1.".to_string() });
        self.brain.train(Language::Spanish, Content { string: "En la carta, los profesionales exponen por qué este tipo de mascarilla puede suponer un 'mayor riesgo': 'Su uso continuado dificulta la respiración, es incómodo, y en determinadas personas, no tolerable. Su utilización en población general favorecerá que las personas se toquen más frecuentemente la cara y se quiten la mascarilla al no poder respirar cómodamente favoreciendo el riesgo de contagio'.".to_string() });
        self.brain.train(Language::Spanish, Content { string: "En este vídeo Santiago García Cremades te da las claves para que no desesperes cuando te topas con una operación matemática de la que no encuentras solución. Algo que puedes necesitar en cualquier ocasión pero que, en estos momentos, cuando muchos se han convertido obligatoriamente en profesores para ayudar a los estudiantes de la casa, se hace de lo más imprescindible.".to_string() });
        self.brain.train(Language::Spanish, Content { string: "En concreto, Santi utiliza su móvil HUAWEI Mate 30 Pro para resolver ecuaciones. Lo hace a través de Microsoft Math Solver, una aplicación que puedes descargar a través de Huawei AppGallery y que, no sólo te mostrará el resultado de tus dudas matemáticas, sino que, aún más importante, te mostrará cómo se ha llegado hasta él. Un claro ejemplo de cómo la tecnología puede convertirse en una gran aliada de la educación en estos días.".to_string() });
        self.brain.train(Language::Spanish, Content { string: "Galicia, el País Vasco, Aragón, Extremadura, parte de Andalucía, Murcia, Canarias, Baleares... la mitad de los españoles entrarán este lunes a la fase 1 del Plan de Desescalada establecido por el Gobierno y, entre otras cosas, eso les permitirá una libertad notable a la hora de hacer deporte. Aquellos corredores y ciclistas que hasta ahora estaban limitados a su propio municipio, ahora podrán disfrutar de los caminos y las carreteras de toda su provincia, su región sanitaria o su isla y el resto de deportistas de disciplinas individuales podrán empezar a ejercitarse en instalaciones al aire libre.".to_string() });
        self.brain.train(Language::Spanish, Content { string: "Los ciudadanos que vivan en zonas en fase 1 podrán regresar a pistas de tenis, de pádel o de atletismo y campos de golf, de tiro o tiro con arco sin importar si estos son de titularidad pública o privada. Lo deberán hacer, eso sí, siguiendo estrictas medidas de seguridad y con ciertos límites: necesitarán cita previa, no podrán utilizar los vestuarios de las instalaciones abiertas y tampoco podrán pararse en las zonas comunes. En casos como los del tenis o el pádel se ha establecido un máximo de dos jugadores por pista mientras en el atletismo sólo se reclama que se cumpla la distancia de seguridad. Para ello, muchos clubes ya están estableciendo horarios estrictos para que los corredores no coincidan sobre el tartán.".to_string() });
        self.brain.train(Language::Spanish, Content { string: "Para los profesionales el cambio también es importante. Deportistas como Rafa Nadal pueden regresar ya a los entrenamientos. Al mismo tiempo los Centros de Alto Rendimiento (CAR) pueden volver a abrir en fase 1, aunque ese cambio apenas afectará a las instalaciones de vela en Santander, el Palma Arena en Mallorca y otros pequeños centros alrededor de España. Los grandes CAR de Madrid, Barcelona, León o Sierra Nevada no podrán reabrir al estar en zonas en fase 0.".to_string() });
        self.brain.train(Language::Spanish, Content { string: "Para volver a utilizar instalaciones bajo techo o las piscinas al aire libre -que han quedado excluidas de la fase 1- habrá que esperar a fase posteriores. De hecho, los gimnasios no volverán a abrir hasta la fase 3. Lo harán entonces sin posibilidad de que se utilicen los vestuarios y con un tercio del aforo. Para que se vuelvan a utilizar pabellones para los deportes de equipo habrá que esperar a que acabe la desescalada, ya en julio.".to_string() });

        // Random French paragraphs from lemonde.fr
        self.brain.train(Language::French, Content { string: "La société doit apprendre à vivre avec le virus. La sortie du confinement et la prévention de rebonds épidémiques sont des moments critiques. Ils doivent mobiliser toutes les expertises de la société française.".to_string() });
        self.brain.train(Language::French, Content { string: "Or la France ne dispose pas d’espace institutionnel pour coordonner le dialogue entre sciences et société. L’efficacité et les conséquences sociales des solutions que proposent sciences et médecine dépendent pourtant du contexte de leur application. L’expérience du terrain et le sens pratique forment aussi un formidable vivier d’idées et de solutions.".to_string() });
        self.brain.train(Language::French, Content { string: "L’émergence, ces dernières semaines, de nouvelles formes de solidarités en est une illustration pleine de promesses. Des solutions ingénieuses et novatrices apparaissent lorsque les enjeux d’un problème sont pensés ensemble. Ainsi, la production de masques artisanaux, destinés aux personnes saines, est aujourd’hui encadrée par une norme de l’Afnor [Association française de normalisation] et une note interministérielle du 29 mars.".to_string() });
        self.brain.train(Language::French, Content { string: "Devant l’évidence des bienfaits de la co-construction, un triptyque vertueux liant société, sciences et politique s’impose. Les signataires de cette tribune appellent à une sortie de crise par le haut grâce à un dialogue soutenu et coordonné entre sciences et société, pour une démocratie sanitaire.".to_string() });
        self.brain.train(Language::French, Content { string: "Parfois critiques des consignes du gouvernement, souvent fiers de la solidarité des habitants de leur commune depuis le 17 mars, quatre élus racontent leur rôle, alors que s’amorce, lundi, un progressif retour à la normale.".to_string() });
        self.brain.train(Language::French, Content { string: "En zone rurale ou en banlieue parisienne, dans un département classé « vert » ou « rouge », dans l’attente d’une réélection ou appelés dès le 15 mars à débuter un nouveau mandat, quatre maires racontent au Monde leurs difficultés, à l’approche du 11 mai, à concevoir sereinement le monde d’après la crise sanitaire.".to_string() });
        self.brain.train(Language::French, Content { string: "A plusieurs reprises, depuis le début de la crise, Jean-Michel Blanquer a semblé en dehors des cercles de décision, au plus haut sommet de l’Etat, en distillant annonces et hypothèses sitôt « recadrées » par l’exécutif, quand elles n’étaient pas carrément démenties. « Il doit avoir les oreilles qui sifflent car en ce moment, il est très critiqué en Macronie, confie une source au sein de l’exécutif. L’Elysée et Matignon trouvent qu’il n’est pas précis et lui reprochent de trop parler, en ouvrant des portes qu’il n’a pas à ouvrir. »".to_string() });
        self.brain.train(Language::French, Content { string: "Rétrospectivement, la séquence pourrait ressembler à un chapelet de « couacs » pour le ministre de l’éducation. Lors de sa première allocution consacrée à l’épidémie, le 12 mars, Emmanuel Macron annonce la fermeture des crèches, écoles, lycées et universités à partir du 16 mars et jusqu’à nouvel ordre. Or, le matin même, Jean-Michel Blanquer écartait cette hypothèse sur Franceinfo : « Nous n’avons jamais envisagé la fermeture totale des écoles car elle nous semble contre-productive », affirmait-il.".to_string() });
        self.brain.train(Language::French, Content { string: "Le compromis de vente était signé, les plans presque bouclés, de l’emplacement de l’âtre dans le salon à la couleur des dalles conduisant à l’immense entrée. « La maison allait ressembler à un magazine de décoration », raconte Philippe*, 50 ans, manager dans une entreprise automobile toulousaine. Le prix mirobolant de la bâtisse – 1,2 million d’euros – n’avait pas freiné son couple, coutumier des « coups » immobiliers et des belles plus-values. « Il y a encore deux mois, on était sûrs que tout ça avait un sens », reconnaît Isabelle, directrice commerciale dans le prêt-à-porter et « toujours entre deux avions ».".to_string() });
        self.brain.train(Language::French, Content { string: "L’un des défis du déconfinement, qui débute à partir de lundi 11 mai, est de soulager les transports en commun tout en évitant de saturer les réseaux routiers. Afin de soutenir le recours par les agents publics et les salariés du privé à des modes de transport alternatifs dès la sortie de la période de confinement, le forfait mobilités durables entrera en vigueur dès lundi, au lieu du 1er juillet (décret et arrêté), a annoncé Oliver Dussopt, secrétaire d’Etat auprès du ministre de l’action et des comptes publics sur son compte Twitter, dimanche 10 mai.".to_string() });
    }
}

fn main() {
    let mut language_detector_bot: LanguageDetector = LanguageDetector::new();
    language_detector_bot.teach();

    println!("Enter English, French, or Spanish text to analyze:");

    let mut input = String::new();

    if let Ok(_) = io::stdin().read_line(&mut input) {
        let to_analyze: Content = Content { string: input };
        let probabilities = language_detector_bot.brain.analyze(to_analyze);

        let mut most_probable: Option<Language> = None;
        for class in language_detector_bot.brain.classes.iter() {
            if let Some(class_probability) = probabilities.get(class) {
                most_probable = match most_probable {
                    Some(current_most_probable_language) => {
                        let mut new_most_probable_language = current_most_probable_language;

                        if let Some(current_highest_probability) = probabilities.get(&current_most_probable_language) {
                            if current_highest_probability < class_probability {
                                new_most_probable_language = *class;
                            }
                        }

                        Some(new_most_probable_language)
                    },
                    None => Some(*class),
                };
            }
        }

        if let Some(probable) = most_probable {
            println!("Language: {}.", probable);
        } else {
            println!("Not enough data.");
        }
    } else {
        println!("Something went wrong.")
    }
}
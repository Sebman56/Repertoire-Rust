use std::io;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::io::{ BufRead, BufReader}; 
use std::path::Path;
use std::process;

use csv::WriterBuilder;
use std::fs::OpenOptions;

use std::error::Error;

fn main() 
{

    let mut nationalite = "NATIONALITE";
    let mut nom = "NOM";
    let mut prenom = "PRENOM";
    let mut coordonnees = "COORDONNEES";
    menu();
}

/************************************************************************************************
 * *** Fonction menu()
 */
fn menu ()-> Result<(), Box<dyn std::error::Error>>{


    println!("\n*************************************************");
    println!("   Repertoire de coordonnees de personnes.    ");
    println!("\tMenu:");
    println!("\t 1-Lister le repertoire.");    
    println!("\t 2-Rechercher dans le repertoire.");
    println!("\t 3-Ajouter dans le repertoire.");
    println!("\t 4-Supprimer une ligne (donner par Rechercher");
    println!("\t 5-Quitter le programme.");
    println!("\n\tChoix:");
    
   // let contents = fs::Choix("ListeCoordonneesPersonnes.txt")?;
    let mut Saisie = String::new();
    // Lire l'entrée de l'utilisateur
    io::stdin()
        .read_line(&mut Saisie )
        .expect("Échec de la lecture de l'entrée");
    // Comparer entrer Y y O o ou non
    let mut  Choix = Saisie.trim().parse().expect("Veuillez entrer un nombre");
    match Choix 
        {
            1 => affichage_fichier(),
            2 => Recherche_chaine(),
            3 => demander_coordonnees_csv(),
            4 => supprimer_ligne(),
            5 => quitter_programme(),
            _ => Ok(()),
        } 
    }
    

/************************************************************************************************
 * *** Fonction pour quitter le programme.
*/
fn quitter_programme()  -> Result<(), Box<dyn Error>>
{
    println!("Au revoir !");
    std::process::exit(0);
}


/************************************************************************************************
 * *** Fonction d'entrer decoodonnées
*/
fn demander_coordonnees_csv() -> Result<(), Box<dyn std::error::Error>> {
    println!("Ajout d'une nouvelle personne :");
    
    println!("Nationalité :");
    let mut nationalite = String::new();
    io::stdin().read_line(&mut nationalite)?;
    
    println!("Nom :");
    let mut nom = String::new();
    io::stdin().read_line(&mut nom)?;
    
    println!("Prénom :");
    let mut prenom = String::new();
    io::stdin().read_line(&mut prenom)?;
    
    println!("Coordonnées :");
    let mut coordonnees = String::new();
    io::stdin().read_line(&mut coordonnees)?;
    
    // Nettoyage des entrées
    let nationalite = nationalite.trim();
    let nom = nom.trim();
    let prenom = prenom.trim();
    let coordonnees = coordonnees.trim();
    
    ajouter_coordonnees_csv(nationalite, nom, prenom, coordonnees)?;
    
    println!("Ajout réussi !");
    menu ();
    Ok(())
}


/*******************************************************************************************
 * *** Ajouter coordonnes dans le fichier ListeCoordonneesPersonnes.txt
 */
fn ajouter_coordonnees_csv(
    nationalite: &str,
    nom: &str,
    prenom: &str,
    coordonnees: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let fichier = "ListeCoordonneesPersonnes.txt";
    
    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(fichier)?;
    
    let mut wtr = WriterBuilder::new()
        .has_headers(false)
        .from_writer(file);
    
    wtr.write_record(&[nationalite, nom, prenom, coordonnees])?;
    wtr.flush()?;
    
    Ok(())
}


/* **********************************************************************************************
*** Recherche d'une chaine de caractere dans le fichier ListeCoordonneesPersonnes.txt
*/
fn Recherche_chaine() -> Result<(), Box<dyn std::error::Error>>  {
    let filename = "ListeCoordonneesPersonnes.txt";
    let search_term = get_search_term()?;

    search_in_file(filename, &search_term)?;
    menu ();
    Ok(())
}

fn get_search_term() -> io::Result<String> {
    println!("Entrez le terme à rechercher :");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn search_in_file(filename: &str, search_term: &str) -> io::Result<()> {
    let path = Path::new(filename);
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    println!("\nRésultats pour '{}':", search_term);

    for (line_num, line) in reader.lines().enumerate() {
        let line = line?;
        if line.contains(search_term) {
            println!("Ligne {}: {}", line_num + 1, line);
        }
    }

    Ok(())
}


/* **********************************************************************************************
*** Affichage ou non du fichier ListeCoordonneesPersonnes si il n'existe pas
*/

fn affichage_fichier() -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string("ListeCoordonneesPersonnes.txt")?;
    let mut Reponse_Affichage_LCP = String::new();
    println!("\n\tVoulez vous afficher le contenu du fichier ListeCoordonneesPersonnes.txt ?");

    // Lire l'entrée de l'utilisateur
    io::stdin()
        .read_line(&mut Reponse_Affichage_LCP)
        .expect("Échec de la lecture de l'entrée");
    // Comparer entrer Y y O o ou non
      let Reponse_Affichage_LCP= Reponse_Affichage_LCP.trim(); // input_string = input_string.trim();
    if ((Reponse_Affichage_LCP == "Y") || (Reponse_Affichage_LCP == "y") || (Reponse_Affichage_LCP == "O") || (Reponse_Affichage_LCP == "o"))
        {
        for line in contents.lines() {
        let parts: Vec<&str> = line.split(',').collect();
        println!("Nationalite: {},\t Nom: {},\tPrenom: {}, \tCoordonnées: {}", parts[0], parts[1],parts[2], parts[3]);
        }
    }
    println!(" \n\tOuverture du fichier réussi !!!\n");
    menu () ;
    Ok(())
}

/* **********************************************************************************************
*** Vérification, création du fichier ListeCoordonneesPersonnes si il n'existe pas
*/
fn verifier_presence_fichier() -> io::Result<()> {
    let filename = "ListeCoordonneesPersonnes.txt";
    
    // Vérifier si le fichier existe
    if !file_exists(filename)? {
        println!("Le fichier n'existe pas. Création...");
        create_file_with_headers(filename)?;
        println!("Fichier créé avec succès.");
    } else {
        println!("Le fichier existe déjà.");
    }
    
    Ok(())
}

fn file_exists(filename: &str) -> io::Result<bool> {
    match fs::metadata(filename) {
        Ok(_) => Ok(true),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(false),
        Err(e) => Err(e),
    }
}

fn create_file_with_headers(filename: &str) -> io::Result<()> {
    let mut file = File::create(filename)?;
    writeln!(file, "Nationalité,Nom,Prénom,Age")?;
    Ok(())
}

/* **********************************************************************************************
***  Fonction pour supprimer une ligne du fichierListeCoordonnéesPersonnes.txt.
*/
fn supprimer_ligne()-> Result<(), Box<dyn std::error::Error>> {
    // Lire le fichier
    let fichier = "ListeCoordonneesPersonnes.txt";
    let path = Path::new(fichier);
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Afficher le contenu avec numéros de ligne
    println!("Contenu actuel du fichier:");
    for (i, line) in reader.lines().enumerate() {
        println!("{}: {}", i + 1, line?);
    }

    // Demander le numéro de ligne à supprimer
    println!("Entrez le numéro de ligne à supprimer:");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let ligne_a_supprimer: usize = input.trim().parse().map_err(|e| {
        io::Error::new(io::ErrorKind::InvalidInput, "Numéro invalide")
    })?;

    // Relire le fichier et reconstruire sans la ligne supprimée
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut nouvelles_lignes = Vec::new();

    for (i, line) in reader.lines().enumerate() {
        if i + 1 != ligne_a_supprimer {
            nouvelles_lignes.push(line?);
        }
    }

    // Réécrire le fichier
    let mut file = File::create(path)?;
    for ligne in nouvelles_lignes {
        writeln!(file, "{}", ligne)?;
    }

    println!("Ligne {} supprimée avec succès!", ligne_a_supprimer);
    menu ();
    Ok(())
}
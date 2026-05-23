
#![no_std]

use soroban_sdk::{
    contract,
    contracterror,
    contractimpl,
    contracttype,
    symbol_short,
    Address,
    Env,
    String,
    Symbol,
    Vec,
};

// =====================================================
// NOTE STRUCT (NOW SUPPORTS MEDIA + COLLAB)
// =====================================================

#[contracttype]
#[derive(Clone, Debug)]
pub struct Note {
    pub id: u64,
    pub owner: Address,
    pub title: String,
    pub content: String,

    // MULTIMEDIA SUPPORT (IPFS / ARWEAVE CID)
    pub media: Vec<String>, // image/video CID list
}

// =====================================================
// ROLE SYSTEM FOR COLLABORATION
// =====================================================

#[contracttype]
#[derive(Clone)]
pub enum Role {
    Owner,
    Editor,
    Viewer,
}

// =====================================================
// COLLABORATION STRUCT
// =====================================================

#[contracttype]
#[derive(Clone)]
pub struct Collaboration {
    pub note_id: u64,
    pub user: Address,
    pub role: Role,
}

// =====================================================
// STORAGE KEY
// =====================================================

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Note(u64),
    UserNotes(Address),
    NoteCollabs(u64),
}

// =====================================================
// ERROR ENUM
// =====================================================

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum NotesError {
    Unauthorized = 1,
    NoteNotFound = 2,
    TitleTooLong = 3,
    ContentTooLong = 4,
    NotEditor = 5,
}

// =====================================================
// CONSTANTS
// =====================================================

const NOTE_COUNTER: Symbol = symbol_short!("COUNTER");

const MAX_TITLE_LENGTH: u32 = 100;
const MAX_CONTENT_LENGTH: u32 = 1000;

const INSTANCE_BUMP_AMOUNT: u32 = 518400;
const INSTANCE_LIFETIME_THRESHOLD: u32 = 518400;

const PERSISTENT_BUMP_AMOUNT: u32 = 518400;
const PERSISTENT_LIFETIME_THRESHOLD: u32 = 518400;

// =====================================================
// CONTRACT
// =====================================================

#[contract]
pub struct NotesContract;

// =====================================================
// IMPLEMENTATION
// =====================================================

#[contractimpl]
impl NotesContract {

    // =================================================
    // CREATE NOTE + AUTO OWNER ROLE
    // =================================================
    pub fn create_note(
        env: Env,
        user: Address,
        title: String,
        content: String,
    ) -> Result<u64, NotesError> {

        user.require_auth();

        if title.len() > MAX_TITLE_LENGTH {
            return Err(NotesError::TitleTooLong);
        }

        if content.len() > MAX_CONTENT_LENGTH {
            return Err(NotesError::ContentTooLong);
        }

        let mut counter: u64 = env.storage()
            .instance()
            .get(&NOTE_COUNTER)
            .unwrap_or(0);

        counter += 1;

        let note = Note {
            id: counter,
            owner: user.clone(),
            title,
            content,
            media: Vec::new(&env),
        };

        // SAVE NOTE
        env.storage()
            .persistent()
            .set(&DataKey::Note(counter), &note);

        // USER NOTES INDEX
        let user_key = DataKey::UserNotes(user.clone());

        let mut user_notes: Vec<u64> = env.storage()
            .persistent()
            .get(&user_key)
            .unwrap_or(Vec::new(&env));

        user_notes.push_back(counter);

        env.storage()
            .persistent()
            .set(&user_key, &user_notes);

        // CREATE DEFAULT COLLAB (OWNER)
        let collab_key = DataKey::NoteCollabs(counter);

        let mut collabs: Vec<Collaboration> = Vec::new(&env);

        collabs.push_back(Collaboration {
            note_id: counter,
            user: user.clone(),
            role: Role::Owner,
        });

        env.storage()
            .persistent()
            .set(&collab_key, &collabs);

        env.storage()
            .instance()
            .set(&NOTE_COUNTER, &counter);

        env.events().publish(
            ("NOTE_CREATED", counter),
            user,
        );

        Ok(counter)
    }

    // =================================================
    // ADD COLLABORATOR
    // =================================================
    pub fn add_collaborator(
        env: Env,
        caller: Address,
        note_id: u64,
        user: Address,
        role: Role,
    ) -> Result<(), NotesError> {

        caller.require_auth();

        let note: Note = env.storage()
            .persistent()
            .get(&DataKey::Note(note_id))
            .ok_or(NotesError::NoteNotFound)?;

        if note.owner != caller {
            return Err(NotesError::Unauthorized);
        }

        let collab_key = DataKey::NoteCollabs(note_id);

        let mut collabs: Vec<Collaboration> = env.storage()
            .persistent()
            .get(&collab_key)
            .unwrap_or(Vec::new(&env));

        collabs.push_back(Collaboration {
            note_id,
            user: user.clone(),
            role,
        });

        env.storage()
            .persistent()
            .set(&collab_key, &collabs);

        Ok(())
    }

    // =================================================
    // DROP IMAGE / VIDEO (MEDIA UPLOAD METADATA)
    // =================================================
    pub fn add_media(
        env: Env,
        caller: Address,
        note_id: u64,
        cid: String,
    ) -> Result<(), NotesError> {

        caller.require_auth();

        let mut note: Note = env.storage()
            .persistent()
            .get(&DataKey::Note(note_id))
            .ok_or(NotesError::NoteNotFound)?;

        // check permission (owner only OR editor)
        if note.owner != caller {

            let collabs: Vec<Collaboration> = env.storage()
                .persistent()
                .get(&DataKey::NoteCollabs(note_id))
                .unwrap_or(Vec::new(&env));

            let mut allowed = false;

            for i in 0..collabs.len() {
                let c = collabs.get(i).unwrap();
                if c.user == caller {
                    if matches!(c.role, Role::Editor | Role::Owner) {
                        allowed = true;
                    }
                }
            }

            if !allowed {
                return Err(NotesError::NotEditor);
            }
        }

        note.media.push_back(cid);

        env.storage()
            .persistent()
            .set(&DataKey::Note(note_id), &note);

        Ok(())
    }

    // =================================================
    // GET NOTE
    // =================================================
    pub fn get_note(
        env: Env,
        id: u64,
    ) -> Result<Note, NotesError> {

        env.storage()
            .persistent()
            .get(&DataKey::Note(id))
            .ok_or(NotesError::NoteNotFound)
    }

    // =================================================
    // GET COLLABORATORS
    // =================================================
    pub fn get_collaborators(
        env: Env,
        note_id: u64,
    ) -> Vec<Collaboration> {

        env.storage()
            .persistent()
            .get(&DataKey::NoteCollabs(note_id))
            .unwrap_or(Vec::new(&env))
    }

    // =================================================
    // DELETE NOTE (OWNER ONLY)
    // =================================================
    pub fn delete_note(
        env: Env,
        caller: Address,
        id: u64,
    ) -> Result<(), NotesError> {

        caller.require_auth();

        let note: Note = env.storage()
            .persistent()
            .get(&DataKey::Note(id))
            .ok_or(NotesError::NoteNotFound)?;

        if note.owner != caller {
            return Err(NotesError::Unauthorized);
        }

        env.storage()
            .persistent()
            .remove(&DataKey::Note(id));

        Ok(())
    }
}

// =====================================================
// TEST MODULE
// =====================================================

mod test;
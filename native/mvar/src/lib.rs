use rustler::resource::ResourceArc;
use rustler::Env;
use rustler::Term;
use rustler_stored_term::StoredTerm;
use std::sync::Mutex;

mod atoms {
    rustler::atoms! {
        ok
    }
}

pub struct MVar {
    inner: Mutex<StoredTerm>,
}

// struct MVarContents {
//     owned_env: OwnedEnv,
//     saved_term: SavedTerm,
// }

impl MVar {
    pub fn new(term: StoredTerm) -> Self {
        Self {
            inner: Mutex::new(term),
        }
    }

    pub fn get<'a>(&self) -> StoredTerm {
        self.inner.lock().unwrap().clone()
    }

    pub fn set(&self, term: StoredTerm) {
        *self.inner.lock().unwrap() = term
    }
}

// impl MVarContents {
//     fn new(term: &Term) -> Self {
//         let owned_env = OwnedEnv::new();
//         let saved_term = owned_env.save(*term);
//         Self {
//             owned_env,
//             saved_term,
//         }
//     }

//     fn get<'a>(&self, env: Env<'a>) -> Term<'a> {
//         self.owned_env.run(|owned_env| {
//             let term = self.saved_term.load(owned_env);
//             term.in_env(env)
//         })
//     }

//     fn set(&mut self, term: &Term) {
//         self.owned_env.clear();
//         self.saved_term = self.owned_env.save(*term);
//     }
// }

#[rustler::nif]
fn new(term: StoredTerm) -> ResourceArc<MVar> {
    ResourceArc::new(MVar::new(term))
}

#[rustler::nif]
fn get(mvar: ResourceArc<MVar>) -> StoredTerm {
    mvar.get()
}

#[rustler::nif]
fn set(env: Env, mvar: ResourceArc<MVar>, term: StoredTerm) -> Term {
    mvar.set(term);
    atoms::ok().to_term(env)
}

fn load(env: Env, _info: Term) -> bool {
    rustler::resource!(MVar, env);
    true
}

rustler::init!("Elixir.MVar", [new, get, set], load = load);

use rustler::env::OwnedEnv;
use rustler::env::SavedTerm;
use rustler::resource::ResourceArc;
use rustler::Env;
use rustler::Term;
use std::sync::Mutex;

pub struct MVar {
    inner: Mutex<MVarContents>,
}

struct MVarContents {
    owned_env: OwnedEnv,
    saved_term: SavedTerm,
}

impl MVar {
    pub fn new(term: &Term) -> Self {
        Self {
            inner: Mutex::new(MVarContents::new(term)),
        }
    }

    pub fn get<'a>(&self, env: Env<'a>) -> Term<'a> {
        self.inner.lock().unwrap().get(env)
    }

    pub fn set(&self, term: &Term) {
        self.inner.lock().unwrap().set(term)
    }
}

impl MVarContents {
    fn new(term: &Term) -> Self {
        let owned_env = OwnedEnv::new();
        let saved_term = owned_env.save(*term);
        Self {
            owned_env,
            saved_term,
        }
    }

    fn get<'a>(&self, env: Env<'a>) -> Term<'a> {
        self.owned_env.run(|owned_env| {
            let term = self.saved_term.load(owned_env);
            term.in_env(env)
        })
    }

    fn set(&mut self, term: &Term) {
        self.owned_env.clear();
        self.saved_term = self.owned_env.save(*term);
    }
}

#[rustler::nif]
fn new(term: Term) -> ResourceArc<MVar> {
    ResourceArc::new(MVar::new(&term))
}

#[rustler::nif]
fn get<'a>(env: Env<'a>, mvar: ResourceArc<MVar>) -> Term<'a> {
    mvar.get(env)
}

#[rustler::nif]
fn set(mvar: ResourceArc<MVar>, term: Term) {
    mvar.set(&term)
}

fn load(env: Env, _info: Term) -> bool {
    rustler::resource!(MVar, env);
    true
}

rustler::init!("Elixir.MVar", [new, get, set], load = load);

use candid::{CandidType,Decode,Deserialize,Encode};
use ic_stable_structures::memory_manager::{MemoryId,MemoryManager,VirtualMemory};
use ic_stable_structures::{BoundedStorable,DefaultMemoryImpl,StableBTreeMap,Storable};
use std::{borrow::Cow,cell::RefCell};
use std::collections::BTreeMap;


type Memory = VirtualMemory<DefaultMemoryImpl>;

const MAX_VALUE_SIZE: u32 = 100;


#[derive(CandidType,Deserialize)]
struct Test{
    over_all:u8,
    subject:String,
    curve:u8,
}
impl Storable for Test{
    fn to_bytes(&self)->std::borrow::Cow<[u8]>{
        Cow::Owned(Encode!(self).unwrap())
    }
    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self{
        Decode!(bytes.as_ref(), Self).unwrap()
    }
   
}

impl BoundedStorable for Test{
    const MAX_SIZE:u32 = MAX_VALUE_SIZE;
    const IS_FIXED_SIZE:bool = false;
}

thread_local!{
    static MEMORY_MANAGER:RefCell<MemoryManager<DefaultMemoryImpl>> =
    RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static TEST_MAP:RefCell<StableBTreeMap<u64,Test,Memory>> = RefCell::new(
    StableBTreeMap::init(
        MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
    )
);
    
    static PERCENTAGE_MAP:RefCell<StableBTreeMap<u64,u64,Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))),
        )
    );
}

#[ic_cdk::query]

fn get_percentage(key: u64) -> Option<u64>{
    PERCENTAGE_MAP.with(|p: &RefCell<BTreeMap<u64,u64,_>>| -> Option<u64>{ p.borrow().get(&key)})
}

#[ic_cdk::query]

fn get_test(key:u64) -> Option<Test> {
    TEST_MAP.with(|p: &RefCell<BTreeMap<u64,Test,_>>|-> Option<Test> { p.borrow_mut().insert(key,value) })
}
#[ic_cdk::update]

fn insert_test(key:u64,value:Test) -> Option<Test> {
    TEST_MAP.with(|p: &RefCell<BTreeMap<u64,Test,_>>| -> Option<Test> { p.borrow_mut().insert(key,value) })
}
#[ic_cdk::update]
fn insert_percentage(key:u64,value:u64) -> Option<u64> {
  PERCENTAGE_MAP.with(|p: &RefCell<BTreeMap<u64,u64,_>>| -> Option<u64> { p.borrow_mut().insert(key,value) })}

use spin::Mutex;

pub struct AcpiTable {
    pub signature: [u8; 4],
    pub length: u32,
    pub data: alloc::vec::Vec<u8>,
}

pub struct AcpiManager {
    tables: Mutex<alloc::collections::BTreeMap<[u8; 4], AcpiTable>>,
    initialized: Mutex<bool>,
}

impl AcpiManager {
    pub const fn new() -> Self {
        AcpiManager {
            tables: Mutex::new(alloc::collections::BTreeMap::new()),
            initialized: Mutex::new(false),
        }
    }

    pub fn init(&self) -> Result<(), &'static str> {
        // TODO: Find and parse ACPI tables
        // For now, just mark as initialized
        *self.initialized.lock() = true;
        crate::io::println!("ACPI: ACPI subsystem initialized");
        Ok(())
    }

    pub fn find_table(&self, signature: &[u8; 4]) -> Option<AcpiTable> {
        self.tables.lock().get(signature).cloned()
    }

    pub fn register_table(&self, table: AcpiTable) {
        self.tables.lock().insert(table.signature, table);
    }

    pub fn get_power_states(&self) -> (u8, u8) {
        // TODO: Read power states from ACPI
        (0, 0) // (current_state, supported_states)
    }
}

pub static ACPI_MANAGER: AcpiManager = AcpiManager::new();


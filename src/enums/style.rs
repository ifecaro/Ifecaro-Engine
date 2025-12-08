#[derive(Debug, Clone, PartialEq)]
pub enum NavbarStyle {
    Link,
    Dropdown,
    DropdownOption,
}

impl NavbarStyle {
    pub fn class(&self) -> &'static str {
        match self {
            NavbarStyle::Link => "flex-1 sm:flex-none text-center text-xs text-gray-800 dark:text-white paper:text-[#1f2937] hover:text-gray-900 dark:hover:text-gray-300 paper:hover:text-[#111827] transition-colors duration-200 py-2 pen-texture-text",
            NavbarStyle::Dropdown => "flex-1 sm:flex-none text-center text-xs text-gray-800 dark:text-white paper:text-[#1f2937] hover:text-gray-900 dark:hover:text-gray-300 paper:hover:text-[#111827] transition-colors duration-200 py-2 cursor-pointer pen-texture-text",
            NavbarStyle::DropdownOption => "text-gray-800 dark:text-gray-300 paper:text-[#374151] hover:bg-gray-100 dark:hover:bg-gray-700 paper:hover:bg-[#f0e6cf] pen-texture-text",
        }
    }
}

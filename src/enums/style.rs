#[derive(Debug, Clone, PartialEq)]
pub enum NavbarStyle {
    Link,
    Dropdown,
    DropdownOption,
}

impl NavbarStyle {
    pub fn class(&self) -> &'static str {
        match self {
            NavbarStyle::Link => "flex-1 sm:flex-none text-center text-xs text-gray-800 dark:text-white paper:text-[#2f2417] hover:text-gray-900 dark:hover:text-gray-300 paper:hover:text-[#1f160e] transition-colors duration-200 py-2",
            NavbarStyle::Dropdown => "flex-1 sm:flex-none text-center text-xs text-gray-800 dark:text-white paper:text-[#2f2417] hover:text-gray-900 dark:hover:text-gray-300 paper:hover:text-[#1f160e] transition-colors duration-200 py-2 cursor-pointer",
            NavbarStyle::DropdownOption => "text-gray-800 dark:text-gray-300 paper:text-[#3f3422] hover:bg-gray-100 dark:hover:bg-gray-700 paper:hover:bg-[#f0e6cf]",
        }
    }
}

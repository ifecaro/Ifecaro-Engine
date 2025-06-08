#[derive(Debug, Clone, PartialEq)]
pub enum NavbarStyle {
    Link,
    Dropdown,
    DropdownOption,
}

impl NavbarStyle {
    pub fn class(&self) -> &'static str {
        match self {
            NavbarStyle::Link => "flex-1 sm:flex-none text-center text-xs text-gray-700 dark:text-white hover:text-gray-900 dark:hover:text-gray-300 transition-colors duration-200 py-2",
            NavbarStyle::Dropdown => "flex-1 sm:flex-none text-center text-xs text-gray-700 dark:text-white hover:text-gray-900 dark:hover:text-gray-300 transition-colors duration-200 py-2 cursor-pointer",
            NavbarStyle::DropdownOption => "text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700",
        }
    }
} 

pub const DROPDOWN_PANEL_CLASS: &str = "absolute right-0 bottom-full sm:bottom-auto sm:top-full mb-2 sm:mb-0 sm:mt-2 w-full sm:min-w-max rounded-md shadow-lg bg-white dark:bg-gray-800 ring-1 ring-black ring-opacity-5 z-[1000] transition duration-200 ease-in-out transform will-change-transform will-change-opacity"; 
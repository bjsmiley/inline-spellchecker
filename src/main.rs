fn main() {
    let text = std::env::args().skip(1).collect::<Vec<String>>().join(" ");
    core::run(text);
}

mod core {
    pub fn run(text: String) {
        let output = check(text);
        println!("{}",output);
    }

    #[cfg(target_os = "windows")]
    pub(crate) fn check(text: String) -> String {
        crate::win::check(text).unwrap()
    }

    #[cfg(not(target_os = "windows"))]
    pub(crate) fn check(text: String) -> String {
        text
    }

    // TODO: other platforms
}

#[cfg(target_os = "windows")]
mod win {
    use windows::{Win32::{
        Globalization::{
            ISpellCheckerFactory, 
            SpellCheckerFactory, 
            CORRECTIVE_ACTION_DELETE, 
            CORRECTIVE_ACTION_REPLACE, 
            CORRECTIVE_ACTION_GET_SUGGESTIONS, 
            CORRECTIVE_ACTION_NONE, 
        }, 
        System::Com::{
            CoInitializeEx, 
            CoCreateInstance, 
            CLSCTX_ALL, 
            COINIT_MULTITHREADED
        }
    }, 
    core::{HSTRING, PWSTR}, w};

    pub fn check(text: String) -> windows::core::Result<String> {

        unsafe { CoInitializeEx(None, COINIT_MULTITHREADED)?; }
        let factory: ISpellCheckerFactory = unsafe { CoCreateInstance(&SpellCheckerFactory, None, CLSCTX_ALL)? };
        let checker = unsafe { factory.CreateSpellChecker(w!("en-US"))? };
        let result = unsafe { checker.ComprehensiveCheck(&HSTRING::from(text.clone()))? };
        let output: &mut Vec<String> = &mut Vec::new();
        let mut current_index: usize = 0;
    
        while let Ok(err) = unsafe { result.Next() } {
            let index = usize::try_from(unsafe { err.StartIndex()? }).unwrap();
            let len = usize::try_from(unsafe { err.Length()? }).unwrap();
            let correction = unsafe { err.CorrectiveAction()? };
    
            let before = &text[current_index..index];
            output.push(before.to_string());
    
            match correction {
                CORRECTIVE_ACTION_REPLACE => {
                    let replacement = unsafe { err.Replacement()?.to_string()? };                
                    output.push(replacement);
                },
                CORRECTIVE_ACTION_GET_SUGGESTIONS => {
                    let word = &text[index..index + len];
                    let suggestions = unsafe { checker.Suggest(&HSTRING::from(word))? };
                    let mut suggestion = [PWSTR::null()];
                    unsafe { let _ = suggestions.Next(&mut suggestion, None); }
                    if suggestion[0].is_null() {
                        output.push(word.to_string());
                    }
                    else {
                        let replacement = unsafe { suggestion[0].display().to_string() };                
                        output.push(replacement);
                    }
                },
                CORRECTIVE_ACTION_NONE => {
                    let word = &text[index..index + len];
                    output.push(word.to_string());
                },
                CORRECTIVE_ACTION_DELETE => {},
                _ => {}
            }
            current_index = index + len;
        }
        output.push((&text[current_index..]).to_string());
        Ok(output.join(""))
    }
}

#[cfg(test)]
mod tests {
    use crate::core;

    #[test]
    fn test_sentence() {
        assert_eq!("Can I  have some?", core::check(String::from("Cann I I haev some?")))
    }

    #[test]
    fn test_word() {
        assert_eq!("dependencies", core::check(String::from("dependencie")))
    }
}
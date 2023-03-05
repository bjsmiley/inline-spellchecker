use windows::{Win32::{Globalization::{ISpellCheckerFactory, SpellCheckerFactory, CORRECTIVE_ACTION_DELETE, CORRECTIVE_ACTION_REPLACE, CORRECTIVE_ACTION_GET_SUGGESTIONS, CORRECTIVE_ACTION_NONE, }, System::Com::{CoInitializeEx, CoCreateInstance, CLSCTX_ALL, COINIT_MULTITHREADED}}, core::{HSTRING, PWSTR}, w};

fn main() -> windows::core::Result<()>{
    
    let utterance = std::env::args().skip(1).collect::<Vec<String>>().join(" ");
    unsafe { CoInitializeEx(None, COINIT_MULTITHREADED)?; }
    let factory: ISpellCheckerFactory = unsafe { CoCreateInstance(&SpellCheckerFactory, None, CLSCTX_ALL)? };
    let checker = unsafe { factory.CreateSpellChecker(w!("en-US"))? };
    let result = unsafe { checker.ComprehensiveCheck(&HSTRING::from(utterance.clone()))? };
    let input = utterance.clone().as_str();
    let output: &mut Vec<String> = &mut Vec::new();
    let mut current_index: usize = 0;

    while let Ok(err) = unsafe { result.Next() } {
        let index = usize::try_from(unsafe { err.StartIndex()? }).unwrap();
        let len = usize::try_from(unsafe { err.Length()? }).unwrap();
        let correction = unsafe { err.CorrectiveAction()? };

        let before = &utterance[current_index..index];
        output.push(before.to_string());

        match correction {
            CORRECTIVE_ACTION_DELETE => {
            },
            CORRECTIVE_ACTION_REPLACE => {
                let replacement = unsafe { err.Replacement()?.to_string()? };                
                output.push(replacement);
            },
            CORRECTIVE_ACTION_GET_SUGGESTIONS => {
                let word = &utterance[index..index + len];
                let suggestions = unsafe { checker.Suggest(&HSTRING::from(word))? };
                let mut suggestion = [PWSTR::null()];
                unsafe { let _ = suggestions.Next(&mut suggestion, None); }
                if suggestion[0].is_null() {
                    output.push(word.to_string());
                }
                else {
                    let replacement = unsafe { err.Replacement()?.to_string()? };                
                    output.push(replacement);
                }
            },
            CORRECTIVE_ACTION_NONE => {
                let word = &utterance[index..index + len];
                output.push(word.to_string());
            },
            _ => {}
        }
        current_index = index + len;
    }

    println!("{}",output.join(""));

    Ok(())
}





pub fn find_substring_in_wrapround_string(p: String) -> i32 {
    p.bytes().fold((vec![0i32;26],0,None),|(mut ans,mut len,last),p_each|{
        if let Some(last)=last{
            let p_each=p_each-97;
            if p_each==(last+1)%26{
                len+=1;
            }else{
                len=1;
            }
            ans[p_each as usize]=ans[p_each as usize].max(len);
            return (ans,len,Some(p_each));
        }else{
            ans[p_each as usize-97]=1;
            return (ans,1,Some(p_each-97));
        }
    }).0.iter().sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        println!(
            "{}",
            crate::find_substring_in_wrapround_string("ab".to_string())
        );
    }

    #[test]
    fn it_works_0() {
        println!(
            "{}",
            crate::find_substring_in_wrapround_string("zaba".to_string())
        );
    }

    #[test]
    fn it_works_1() {
        println!(
            "{}",
            crate::find_substring_in_wrapround_string("aab".to_string())
        );
    }

    #[test]
    fn it_works_2() {
        println!(
            "{}",
            crate::find_substring_in_wrapround_string(
                "nopqrstunopqrstwxklmnopqrstuvwxabcdefghibcbcdefghijkbcdefghijklcdefbcdelmnopqrstuvwxylmnopqrstuvcdefghijnopqrsbcdefghijklmnopqrspqrstucdefqrstuvwijklmfghijklmnopqrstuvwxyzrstuvwxhifghijklmnopqrstuvwxystuvwxylmnopdefghijklmnoijklmnopqrstuvwxabcdefgghijklmnfghijkmeftuvwxyzijkklmnopqrstughijklmnopqrstunopqrpqrstuvwxyzstklmnopjklmdefjklmnoqrstuvwbcdefghstuvwklmnopqrstubcdeijklmnopqrstuvwxylmnohijklmnopqrstuvwxyzbcdefghijklmnopqnopqrstghijklmnopqbcdefghijpqrstuvcdefghijklmnopqrstuvwwxyznopqrstuvwxycdefghijrshiefghijklmnopqrstcdefghijklmnopqrefghicdefhijjklmnopqrjklmnopqrstuvwfghijklmnopqrstuvefghijklmnfgbcdefghijklefghijklmnopqrstuvwxygghijklmnophijklmnopqrstefghijklmnopqrstuvwxyzopqrstuvwxyzqrstuvfghijklmnopbcdefghijklmnopqrstuvwxfghijcdefghijklmnopqrstuvwxyklmnopqrstabcdefghijabcabcdefstuabcdefghijklmnopqrfgvwxyhijklmnolmnopqrstudefghijklmnopqrhijklmnophijklmjklmnopqr"
                .to_string()
            )
        );
    }
}

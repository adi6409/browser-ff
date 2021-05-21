//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/satchel/nsIInputListAutoComplete.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIInputListAutoComplete",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* nsIAutoCompleteResult autoCompleteSearch (in AString aSearchString, in HTMLInputElement aField); */
                    Method {
                        name: "AutoCompleteSearch",
                        params: &[Param { name: "aSearchString", ty: "*const ::nsstring::nsAString" }, Param { name: "aField", ty: "*const libc::c_void" }, Param { name: "_retval", ty: "*mut*const nsIAutoCompleteResult" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}


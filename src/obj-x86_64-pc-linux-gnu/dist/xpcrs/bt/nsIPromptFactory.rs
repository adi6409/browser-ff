//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/windowwatcher/nsIPromptFactory.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPromptFactory",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void getPrompt (in mozIDOMWindowProxy aParent, in nsIIDRef iid, [iid_is (iid), retval] out nsQIResult result); */
                    Method {
                        name: "GetPrompt",
                        params: &[Param { name: "aParent", ty: "*const mozIDOMWindowProxy" }, Param { name: "iid", ty: "*const nsIID" }, Param { name: "result", ty: "*mut *mut libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}


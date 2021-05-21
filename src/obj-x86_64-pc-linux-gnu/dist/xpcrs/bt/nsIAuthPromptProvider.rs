//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIAuthPromptProvider.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAuthPromptProvider",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void getAuthPrompt (in uint32_t aPromptReason, in nsIIDRef iid, [iid_is (iid), retval] out nsQIResult result); */
                    Method {
                        name: "GetAuthPrompt",
                        params: &[Param { name: "aPromptReason", ty: "uint32_t" }, Param { name: "iid", ty: "*const nsIID" }, Param { name: "result", ty: "*mut *mut libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}


//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIDeprecationWarner.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDeprecationWarner",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void issueWarning (in uint32_t aWarning, [optional] in bool aAsError); */
                    Method {
                        name: "IssueWarning",
                        params: &[Param { name: "aWarning", ty: "uint32_t" }, Param { name: "aAsError", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}


//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/urlformatter/nsIURLFormatter.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIURLFormatter",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* AString formatURL (in AString aFormat); */
                    Method {
                        name: "FormatURL",
                        params: &[Param { name: "aFormat", ty: "*const ::nsstring::nsAString" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString formatURLPref (in AString aPref); */
                    Method {
                        name: "FormatURLPref",
                        params: &[Param { name: "aPref", ty: "*const ::nsstring::nsAString" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString trimSensitiveURLs (in AString aMsg); */
                    Method {
                        name: "TrimSensitiveURLs",
                        params: &[Param { name: "aMsg", ty: "*const ::nsstring::nsAString" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}


//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/ds/nsIINIParser.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIINIParser",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* nsIUTF8StringEnumerator getSections (); */
                    Method {
                        name: "GetSections",
                        params: &[Param { name: "_retval", ty: "*mut*const nsIUTF8StringEnumerator" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIUTF8StringEnumerator getKeys (in AUTF8String aSection); */
                    Method {
                        name: "GetKeys",
                        params: &[Param { name: "aSection", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut*const nsIUTF8StringEnumerator" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AUTF8String getString (in AUTF8String aSection, in AUTF8String aKey); */
                    Method {
                        name: "GetString",
                        params: &[Param { name: "aSection", ty: "*const ::nsstring::nsACString" }, Param { name: "aKey", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIINIParserWriter",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void setString (in AUTF8String aSection, in AUTF8String aKey, in AUTF8String aValue); */
                    Method {
                        name: "SetString",
                        params: &[Param { name: "aSection", ty: "*const ::nsstring::nsACString" }, Param { name: "aKey", ty: "*const ::nsstring::nsACString" }, Param { name: "aValue", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void writeFile (in nsIFile aINIFile); */
                    Method {
                        name: "WriteFile",
                        params: &[Param { name: "aINIFile", ty: "*const nsIFile" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIINIParserFactory",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* nsIINIParser createINIParser ([optional] in nsIFile aINIFile); */
                    Method {
                        name: "CreateINIParser",
                        params: &[Param { name: "aINIFile", ty: "*const nsIFile" }, Param { name: "_retval", ty: "*mut *const nsIINIParser" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}


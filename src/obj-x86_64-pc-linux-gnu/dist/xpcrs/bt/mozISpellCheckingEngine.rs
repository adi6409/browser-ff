//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/extensions/spellcheck/idl/mozISpellCheckingEngine.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "mozISpellCheckingEngine",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* attribute ACString dictionary; */
                    Method {
                        name: "GetDictionary",
                        params: &[Param { name: "aDictionary", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetDictionary",
                        params: &[Param { name: "aDictionary", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute mozIPersonalDictionary personalDictionary; */
                    Method {
                        name: "GetPersonalDictionary",
                        params: &[Param { name: "aPersonalDictionary", ty: "*mut*const mozIPersonalDictionary" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetPersonalDictionary",
                        params: &[Param { name: "aPersonalDictionary", ty: "*const mozIPersonalDictionary" }],
                        ret: "::nserror::nsresult",
                    },

                    /* Array<ACString> getDictionaryList (); */
                    Method {
                        name: "GetDictionaryList",
                        params: &[Param { name: "_retval", ty: "*mut thin_vec::ThinVec<::nsstring::nsCString>" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean check (in AString word); */
                    Method {
                        name: "Check",
                        params: &[Param { name: "word", ty: "*const ::nsstring::nsAString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* Array<AString> suggest (in AString word); */
                    Method {
                        name: "Suggest",
                        params: &[Param { name: "word", ty: "*const ::nsstring::nsAString" }, Param { name: "_retval", ty: "*mut thin_vec::ThinVec<::nsstring::nsString>" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void loadDictionariesFromDir (in nsIFile dir); */
                    Method {
                        name: "LoadDictionariesFromDir",
                        params: &[Param { name: "dir", ty: "*const nsIFile" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void addDirectory (in nsIFile dir); */
                    Method {
                        name: "AddDirectory",
                        params: &[Param { name: "dir", ty: "*const nsIFile" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void removeDirectory (in nsIFile dir); */
                    Method {
                        name: "RemoveDirectory",
                        params: &[Param { name: "dir", ty: "*const nsIFile" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void addDictionary (in AString lang, in nsIURI file); */
                    Method {
                        name: "AddDictionary",
                        params: &[Param { name: "lang", ty: "*const ::nsstring::nsAString" }, Param { name: "file", ty: "*const nsIURI" }],
                        ret: "::nserror::nsresult",
                    },

                    /* bool removeDictionary (in AString lang, in nsIURI file); */
                    Method {
                        name: "RemoveDictionary",
                        params: &[Param { name: "lang", ty: "*const ::nsstring::nsAString" }, Param { name: "file", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}


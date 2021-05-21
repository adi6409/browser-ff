//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/commandlines/nsICommandLine.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsICommandLine",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute long length; */
                    Method {
                        name: "GetLength",
                        params: &[Param { name: "aLength", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString getArgument (in long aIndex); */
                    Method {
                        name: "GetArgument",
                        params: &[Param { name: "aIndex", ty: "i32" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* long findFlag (in AString aFlag, in boolean aCaseSensitive); */
                    Method {
                        name: "FindFlag",
                        params: &[Param { name: "aFlag", ty: "*const ::nsstring::nsAString" }, Param { name: "aCaseSensitive", ty: "bool" }, Param { name: "_retval", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void removeArguments (in long aStart, in long aEnd); */
                    Method {
                        name: "RemoveArguments",
                        params: &[Param { name: "aStart", ty: "i32" }, Param { name: "aEnd", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean handleFlag (in AString aFlag, in boolean aCaseSensitive); */
                    Method {
                        name: "HandleFlag",
                        params: &[Param { name: "aFlag", ty: "*const ::nsstring::nsAString" }, Param { name: "aCaseSensitive", ty: "bool" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString handleFlagWithParam (in AString aFlag, in boolean aCaseSensitive); */
                    Method {
                        name: "HandleFlagWithParam",
                        params: &[Param { name: "aFlag", ty: "*const ::nsstring::nsAString" }, Param { name: "aCaseSensitive", ty: "bool" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long state; */
                    Method {
                        name: "GetState",
                        params: &[Param { name: "aState", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute boolean preventDefault; */
                    Method {
                        name: "GetPreventDefault",
                        params: &[Param { name: "aPreventDefault", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetPreventDefault",
                        params: &[Param { name: "aPreventDefault", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIFile workingDirectory; */
                    Method {
                        name: "GetWorkingDirectory",
                        params: &[Param { name: "aWorkingDirectory", ty: "*mut*const nsIFile" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIFile resolveFile (in AString aArgument); */
                    Method {
                        name: "ResolveFile",
                        params: &[Param { name: "aArgument", ty: "*const ::nsstring::nsAString" }, Param { name: "_retval", ty: "*mut*const nsIFile" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIURI resolveURI (in AString aArgument); */
                    Method {
                        name: "ResolveURI",
                        params: &[Param { name: "aArgument", ty: "*const ::nsstring::nsAString" }, Param { name: "_retval", ty: "*mut*const nsIURI" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}


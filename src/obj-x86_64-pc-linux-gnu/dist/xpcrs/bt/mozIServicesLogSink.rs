//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/services/interfaces/mozIServicesLogSink.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "mozIServicesLogSink",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* attribute short maxLevel; */
                    Method {
                        name: "GetMaxLevel",
                        params: &[Param { name: "aMaxLevel", ty: "*mut i16" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetMaxLevel",
                        params: &[Param { name: "aMaxLevel", ty: "i16" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void error (in AString message); */
                    Method {
                        name: "Error",
                        params: &[Param { name: "message", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void warn (in AString message); */
                    Method {
                        name: "Warn",
                        params: &[Param { name: "message", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void debug (in AString message); */
                    Method {
                        name: "Debug",
                        params: &[Param { name: "message", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void trace (in AString message); */
                    Method {
                        name: "Trace",
                        params: &[Param { name: "message", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void info (in AString message); */
                    Method {
                        name: "Info",
                        params: &[Param { name: "message", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}


//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIRequest.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIRequest",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute AUTF8String name; */
                    Method {
                        name: "GetName",
                        params: &[Param { name: "aName", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean isPending (); */
                    Method {
                        name: "IsPending",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsresult status; */
                    Method {
                        name: "GetStatus",
                        params: &[Param { name: "aStatus", ty: "*mut ::nserror::nsresult" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void cancel (in nsresult aStatus); */
                    Method {
                        name: "Cancel",
                        params: &[Param { name: "aStatus", ty: "::nserror::nsresult" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void suspend (); */
                    Method {
                        name: "Suspend",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void resume (); */
                    Method {
                        name: "Resume",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute nsILoadGroup loadGroup; */
                    Method {
                        name: "GetLoadGroup",
                        params: &[Param { name: "aLoadGroup", ty: "*mut*const nsILoadGroup" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetLoadGroup",
                        params: &[Param { name: "aLoadGroup", ty: "*const nsILoadGroup" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute nsLoadFlags loadFlags; */
                    Method {
                        name: "GetLoadFlags",
                        params: &[Param { name: "aLoadFlags", ty: "*mut nsLoadFlags" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetLoadFlags",
                        params: &[Param { name: "aLoadFlags", ty: "nsLoadFlags" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIRequest_TRRMode getTRRMode (); */
                    Method {
                        name: "GetTRRMode",
                        params: &[Param { name: "_retval", ty: "*mut u8" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setTRRMode (in nsIRequest_TRRMode mode); */
                    Method {
                        name: "SetTRRMode",
                        params: &[Param { name: "mode", ty: " u8" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}


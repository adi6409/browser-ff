//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/media/nsIDOMNavigatorUserMedia.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIMediaDevice",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute AString type; */
                    Method {
                        name: "GetType",
                        params: &[Param { name: "aType", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString name; */
                    Method {
                        name: "GetName",
                        params: &[Param { name: "aName", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString id; */
                    Method {
                        name: "GetId",
                        params: &[Param { name: "aId", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString mediaSource; */
                    Method {
                        name: "GetMediaSource",
                        params: &[Param { name: "aMediaSource", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString rawId; */
                    Method {
                        name: "GetRawId",
                        params: &[Param { name: "aRawId", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString groupId; */
                    Method {
                        name: "GetGroupId",
                        params: &[Param { name: "aGroupId", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString rawGroupId; */
                    Method {
                        name: "GetRawGroupId",
                        params: &[Param { name: "aRawGroupId", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean scary; */
                    Method {
                        name: "GetScary",
                        params: &[Param { name: "aScary", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString rawName; */
                    Method {
                        name: "GetRawName",
                        params: &[Param { name: "aRawName", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}


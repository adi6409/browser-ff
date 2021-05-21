//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessibleAnnouncementEvent.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAccessibleAnnouncementEvent",
            base: Some("nsIAccessibleEvent"),
            methods: Ok(&[
                    /* readonly attribute AString announcement; */
                    Method {
                        name: "GetAnnouncement",
                        params: &[Param { name: "aAnnouncement", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned short priority; */
                    Method {
                        name: "GetPriority",
                        params: &[Param { name: "aPriority", ty: "*mut u16" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}


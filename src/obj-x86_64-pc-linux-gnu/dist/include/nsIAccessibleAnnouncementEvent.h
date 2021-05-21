/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessibleAnnouncementEvent.idl
 */

#ifndef __gen_nsIAccessibleAnnouncementEvent_h__
#define __gen_nsIAccessibleAnnouncementEvent_h__


#ifndef __gen_nsIAccessibleEvent_h__
#include "nsIAccessibleEvent.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIAccessibleAnnouncementEvent */
#define NS_IACCESSIBLEANNOUNCEMENTEVENT_IID_STR "8818e49c-1286-4fe6-ae82-4d1b795ec88d"

#define NS_IACCESSIBLEANNOUNCEMENTEVENT_IID \
  {0x8818e49c, 0x1286, 0x4fe6, \
    { 0xae, 0x82, 0x4d, 0x1b, 0x79, 0x5e, 0xc8, 0x8d }}

class NS_NO_VTABLE nsIAccessibleAnnouncementEvent : public nsIAccessibleEvent {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IACCESSIBLEANNOUNCEMENTEVENT_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIAccessibleAnnouncementEvent;

  enum {
    POLITE = 0U,
    ASSERTIVE = 1U
  };

  /* readonly attribute AString announcement; */
  NS_IMETHOD GetAnnouncement(nsAString& aAnnouncement) = 0;

  /* readonly attribute unsigned short priority; */
  NS_IMETHOD GetPriority(uint16_t *aPriority) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIAccessibleAnnouncementEvent, NS_IACCESSIBLEANNOUNCEMENTEVENT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIACCESSIBLEANNOUNCEMENTEVENT \
  NS_IMETHOD GetAnnouncement(nsAString& aAnnouncement) override; \
  NS_IMETHOD GetPriority(uint16_t *aPriority) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIACCESSIBLEANNOUNCEMENTEVENT \
  nsresult GetAnnouncement(nsAString& aAnnouncement); \
  nsresult GetPriority(uint16_t *aPriority); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIACCESSIBLEANNOUNCEMENTEVENT(_to) \
  NS_IMETHOD GetAnnouncement(nsAString& aAnnouncement) override { return _to GetAnnouncement(aAnnouncement); } \
  NS_IMETHOD GetPriority(uint16_t *aPriority) override { return _to GetPriority(aPriority); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIACCESSIBLEANNOUNCEMENTEVENT(_to) \
  NS_IMETHOD GetAnnouncement(nsAString& aAnnouncement) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAnnouncement(aAnnouncement); } \
  NS_IMETHOD GetPriority(uint16_t *aPriority) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPriority(aPriority); } 


#endif /* __gen_nsIAccessibleAnnouncementEvent_h__ */

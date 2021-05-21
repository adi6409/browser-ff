/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIResumableChannel.idl
 */

#ifndef __gen_nsIResumableChannel_h__
#define __gen_nsIResumableChannel_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIStreamListener; /* forward declaration */


/* starting interface:    nsIResumableChannel */
#define NS_IRESUMABLECHANNEL_IID_STR "4ad136fa-83af-4a22-a76e-503642c0f4a8"

#define NS_IRESUMABLECHANNEL_IID \
  {0x4ad136fa, 0x83af, 0x4a22, \
    { 0xa7, 0x6e, 0x50, 0x36, 0x42, 0xc0, 0xf4, 0xa8 }}

class NS_NO_VTABLE nsIResumableChannel : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IRESUMABLECHANNEL_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIResumableChannel;

  /* void resumeAt (in unsigned long long startPos, in ACString entityID); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ResumeAt(uint64_t startPos, const nsACString& entityID) = 0;

  /* readonly attribute ACString entityID; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetEntityID(nsACString& aEntityID) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIResumableChannel, NS_IRESUMABLECHANNEL_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIRESUMABLECHANNEL \
  NS_IMETHOD ResumeAt(uint64_t startPos, const nsACString& entityID) override; \
  NS_IMETHOD GetEntityID(nsACString& aEntityID) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIRESUMABLECHANNEL \
  nsresult ResumeAt(uint64_t startPos, const nsACString& entityID); \
  nsresult GetEntityID(nsACString& aEntityID); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIRESUMABLECHANNEL(_to) \
  NS_IMETHOD ResumeAt(uint64_t startPos, const nsACString& entityID) override { return _to ResumeAt(startPos, entityID); } \
  NS_IMETHOD GetEntityID(nsACString& aEntityID) override { return _to GetEntityID(aEntityID); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIRESUMABLECHANNEL(_to) \
  NS_IMETHOD ResumeAt(uint64_t startPos, const nsACString& entityID) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ResumeAt(startPos, entityID); } \
  NS_IMETHOD GetEntityID(nsACString& aEntityID) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetEntityID(aEntityID); } 


#endif /* __gen_nsIResumableChannel_h__ */

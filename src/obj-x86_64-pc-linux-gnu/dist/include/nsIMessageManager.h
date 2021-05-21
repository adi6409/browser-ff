/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/base/nsIMessageManager.idl
 */

#ifndef __gen_nsIMessageManager_h__
#define __gen_nsIMessageManager_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIContent; /* forward declaration */


/* starting interface:    nsIMessageSender */
#define NS_IMESSAGESENDER_IID_STR "bb5d79e4-e73c-45e7-9651-4d718f4b994c"

#define NS_IMESSAGESENDER_IID \
  {0xbb5d79e4, 0xe73c, 0x45e7, \
    { 0x96, 0x51, 0x4d, 0x71, 0x8f, 0x4b, 0x99, 0x4c }}

class NS_NO_VTABLE nsIMessageSender : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IMESSAGESENDER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIMessageSender;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIMessageSender, NS_IMESSAGESENDER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIMESSAGESENDER \
  /* no methods! */

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIMESSAGESENDER \
  /* no methods! */

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIMESSAGESENDER(_to) \
  /* no methods! */

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIMESSAGESENDER(_to) \
  /* no methods! */


/* starting interface:    nsIInProcessContentFrameMessageManager */
#define NS_IINPROCESSCONTENTFRAMEMESSAGEMANAGER_IID_STR "b39a3324-b574-4f85-8cdb-274d04f807ef"

#define NS_IINPROCESSCONTENTFRAMEMESSAGEMANAGER_IID \
  {0xb39a3324, 0xb574, 0x4f85, \
    { 0x8c, 0xdb, 0x27, 0x4d, 0x04, 0xf8, 0x07, 0xef }}

class NS_NO_VTABLE nsIInProcessContentFrameMessageManager : public nsIMessageSender {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IINPROCESSCONTENTFRAMEMESSAGEMANAGER_IID)

  /* [notxpcom] nsIContent getOwnerContent (); */
  NS_IMETHOD_(nsIContent *) GetOwnerContent(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIInProcessContentFrameMessageManager, NS_IINPROCESSCONTENTFRAMEMESSAGEMANAGER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIINPROCESSCONTENTFRAMEMESSAGEMANAGER \
  NS_IMETHOD_(nsIContent *) GetOwnerContent(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIINPROCESSCONTENTFRAMEMESSAGEMANAGER \
  nsresult_(nsIContent *) GetOwnerContent(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIINPROCESSCONTENTFRAMEMESSAGEMANAGER(_to) \
  NS_IMETHOD_(nsIContent *) GetOwnerContent(void) override { return _to GetOwnerContent(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIINPROCESSCONTENTFRAMEMESSAGEMANAGER(_to) \
  NS_IMETHOD_(nsIContent *) GetOwnerContent(void) override; 


#endif /* __gen_nsIMessageManager_h__ */

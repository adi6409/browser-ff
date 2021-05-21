/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/file/ipc/mozIRemoteLazyInputStream.idl
 */

#ifndef __gen_mozIRemoteLazyInputStream_h__
#define __gen_mozIRemoteLazyInputStream_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIInputStream; /* forward declaration */


/* starting interface:    mozIRemoteLazyInputStream */
#define MOZIREMOTELAZYINPUTSTREAM_IID_STR "4125585f-b0c2-4964-a83c-4b0d99f26d49"

#define MOZIREMOTELAZYINPUTSTREAM_IID \
  {0x4125585f, 0xb0c2, 0x4964, \
    { 0xa8, 0x3c, 0x4b, 0x0d, 0x99, 0xf2, 0x6d, 0x49 }}

class NS_NO_VTABLE mozIRemoteLazyInputStream : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(MOZIREMOTELAZYINPUTSTREAM_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = mozIRemoteLazyInputStream;

  /* [noscript,notxpcom] nsIInputStream GetInternalStream (); */
  NS_IMETHOD_(nsIInputStream *) GetInternalStream(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(mozIRemoteLazyInputStream, MOZIREMOTELAZYINPUTSTREAM_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_MOZIREMOTELAZYINPUTSTREAM \
  NS_IMETHOD_(nsIInputStream *) GetInternalStream(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_MOZIREMOTELAZYINPUTSTREAM \
  nsresult_(nsIInputStream *) GetInternalStream(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_MOZIREMOTELAZYINPUTSTREAM(_to) \
  NS_IMETHOD_(nsIInputStream *) GetInternalStream(void) override { return _to GetInternalStream(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_MOZIREMOTELAZYINPUTSTREAM(_to) \
  NS_IMETHOD_(nsIInputStream *) GetInternalStream(void) override; 


#endif /* __gen_mozIRemoteLazyInputStream_h__ */

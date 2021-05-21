/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/image/imgINotificationObserver.idl
 */

#ifndef __gen_imgINotificationObserver_h__
#define __gen_imgINotificationObserver_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class imgIRequest; /* forward declaration */

#include "nsRect.h"

/* starting interface:    imgINotificationObserver */
#define IMGINOTIFICATIONOBSERVER_IID_STR "03da5641-a333-454a-a859-036d0bb683b7"

#define IMGINOTIFICATIONOBSERVER_IID \
  {0x03da5641, 0xa333, 0x454a, \
    { 0xa8, 0x59, 0x03, 0x6d, 0x0b, 0xb6, 0x83, 0xb7 }}

class NS_NO_VTABLE imgINotificationObserver : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(IMGINOTIFICATIONOBSERVER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = imgINotificationObserver;

  enum {
    SIZE_AVAILABLE = 1,
    FRAME_UPDATE = 2,
    FRAME_COMPLETE = 3,
    LOAD_COMPLETE = 4,
    DECODE_COMPLETE = 5,
    DISCARD = 6,
    UNLOCKED_DRAW = 7,
    IS_ANIMATED = 8,
    HAS_TRANSPARENCY = 9
  };

  /* [noscript,nostdcall,notxpcom] void notify (in imgIRequest aProxy, in long aType, [const] in nsIntRect aRect); */
  virtual void Notify(imgIRequest *aProxy, int32_t aType, const nsIntRect * aRect) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(imgINotificationObserver, IMGINOTIFICATIONOBSERVER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_IMGINOTIFICATIONOBSERVER \
  virtual void Notify(imgIRequest *aProxy, int32_t aType, const nsIntRect * aRect) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_IMGINOTIFICATIONOBSERVER \
  void Notify(imgIRequest *aProxy, int32_t aType, const nsIntRect * aRect); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_IMGINOTIFICATIONOBSERVER(_to) \
  virtual void Notify(imgIRequest *aProxy, int32_t aType, const nsIntRect * aRect) override { return _to Notify(aProxy, aType, aRect); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_IMGINOTIFICATIONOBSERVER(_to) \
  virtual void Notify(imgIRequest *aProxy, int32_t aType, const nsIntRect * aRect) override; 


#endif /* __gen_imgINotificationObserver_h__ */

/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/threads/nsIRunnable.idl
 */

#ifndef __gen_nsIRunnable_h__
#define __gen_nsIRunnable_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIRunnable */
#define NS_IRUNNABLE_IID_STR "4a2abaf0-6886-11d3-9382-00104ba0fd40"

#define NS_IRUNNABLE_IID \
  {0x4a2abaf0, 0x6886, 0x11d3, \
    { 0x93, 0x82, 0x00, 0x10, 0x4b, 0xa0, 0xfd, 0x40 }}

class NS_NO_VTABLE nsIRunnable : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IRUNNABLE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIRunnable;

  /* void run (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Run(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIRunnable, NS_IRUNNABLE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIRUNNABLE \
  NS_IMETHOD Run(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIRUNNABLE \
  nsresult Run(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIRUNNABLE(_to) \
  NS_IMETHOD Run(void) override { return _to Run(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIRUNNABLE(_to) \
  NS_IMETHOD Run(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Run(); } 


/* starting interface:    nsIRunnablePriority */
#define NS_IRUNNABLEPRIORITY_IID_STR "e75aa42a-80a9-11e6-afb5-e89d87348e2c"

#define NS_IRUNNABLEPRIORITY_IID \
  {0xe75aa42a, 0x80a9, 0x11e6, \
    { 0xaf, 0xb5, 0xe8, 0x9d, 0x87, 0x34, 0x8e, 0x2c }}

class NS_NO_VTABLE nsIRunnablePriority : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IRUNNABLEPRIORITY_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIRunnablePriority;

  enum {
    PRIORITY_IDLE = 0U,
    PRIORITY_DEFERRED_TIMERS = 1U,
    PRIORITY_NORMAL = 3U,
    PRIORITY_MEDIUMHIGH = 4U,
    PRIORITY_INPUT_HIGH = 5U,
    PRIORITY_HIGH = 6U
  };

  /* readonly attribute unsigned long priority; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPriority(uint32_t *aPriority) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIRunnablePriority, NS_IRUNNABLEPRIORITY_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIRUNNABLEPRIORITY \
  NS_IMETHOD GetPriority(uint32_t *aPriority) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIRUNNABLEPRIORITY \
  nsresult GetPriority(uint32_t *aPriority); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIRUNNABLEPRIORITY(_to) \
  NS_IMETHOD GetPriority(uint32_t *aPriority) override { return _to GetPriority(aPriority); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIRUNNABLEPRIORITY(_to) \
  NS_IMETHOD GetPriority(uint32_t *aPriority) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPriority(aPriority); } 


/* starting interface:    nsIRunnableIPCMessageType */
#define NS_IRUNNABLEIPCMESSAGETYPE_IID_STR "3114c36c-a482-4c6e-9523-1dcfc6f605b9"

#define NS_IRUNNABLEIPCMESSAGETYPE_IID \
  {0x3114c36c, 0xa482, 0x4c6e, \
    { 0x95, 0x23, 0x1d, 0xcf, 0xc6, 0xf6, 0x05, 0xb9 }}

class NS_NO_VTABLE nsIRunnableIPCMessageType : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IRUNNABLEIPCMESSAGETYPE_IID)

  /* readonly attribute unsigned long type; */
  NS_IMETHOD GetType(uint32_t *aType) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIRunnableIPCMessageType, NS_IRUNNABLEIPCMESSAGETYPE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIRUNNABLEIPCMESSAGETYPE \
  NS_IMETHOD GetType(uint32_t *aType) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIRUNNABLEIPCMESSAGETYPE \
  nsresult GetType(uint32_t *aType); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIRUNNABLEIPCMESSAGETYPE(_to) \
  NS_IMETHOD GetType(uint32_t *aType) override { return _to GetType(aType); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIRUNNABLEIPCMESSAGETYPE(_to) \
  NS_IMETHOD GetType(uint32_t *aType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetType(aType); } 


#endif /* __gen_nsIRunnable_h__ */

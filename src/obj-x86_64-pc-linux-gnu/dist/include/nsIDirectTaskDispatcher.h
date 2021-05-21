/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/threads/nsIDirectTaskDispatcher.idl
 */

#ifndef __gen_nsIDirectTaskDispatcher_h__
#define __gen_nsIDirectTaskDispatcher_h__


#ifndef __gen_nsIRunnable_h__
#include "nsIRunnable.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
#include "mozilla/AlreadyAddRefed.h"

/* starting interface:    nsIDirectTaskDispatcher */
#define NS_IDIRECTTASKDISPATCHER_IID_STR "e05bf0fe-94b7-4e28-8462-a8368da9c136"

#define NS_IDIRECTTASKDISPATCHER_IID \
  {0xe05bf0fe, 0x94b7, 0x4e28, \
    { 0x84, 0x62, 0xa8, 0x36, 0x8d, 0xa9, 0xc1, 0x36 }}

class nsIDirectTaskDispatcher : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IDIRECTTASKDISPATCHER_IID)

  /* [noscript] void dispatchDirectTask (in alreadyAddRefed_nsIRunnable event); */
  NS_IMETHOD DispatchDirectTask(already_AddRefed<nsIRunnable> event) = 0;

  /* [noscript] void drainDirectTasks (); */
  NS_IMETHOD DrainDirectTasks(void) = 0;

  /* [noscript] bool haveDirectTasks (); */
  NS_IMETHOD HaveDirectTasks(bool *_retval) = 0;

   // Infallible version of the above. Will assert that it is successful.
  bool HaveDirectTasks() {
    bool value = false;
    MOZ_ALWAYS_SUCCEEDS(HaveDirectTasks(&value));
    return value;
  }
  };

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIDirectTaskDispatcher, NS_IDIRECTTASKDISPATCHER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIDIRECTTASKDISPATCHER \
  NS_IMETHOD DispatchDirectTask(already_AddRefed<nsIRunnable> event) override; \
  NS_IMETHOD DrainDirectTasks(void) override; \
  NS_IMETHOD HaveDirectTasks(bool *_retval) override; \

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIDIRECTTASKDISPATCHER \
  nsresult DispatchDirectTask(already_AddRefed<nsIRunnable> event); \
  nsresult DrainDirectTasks(void); \
  nsresult HaveDirectTasks(bool *_retval); \

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIDIRECTTASKDISPATCHER(_to) \
  NS_IMETHOD DispatchDirectTask(already_AddRefed<nsIRunnable> event) override { return _to DispatchDirectTask(event); } \
  NS_IMETHOD DrainDirectTasks(void) override { return _to DrainDirectTasks(); } \
  NS_IMETHOD HaveDirectTasks(bool *_retval) override { return _to HaveDirectTasks(_retval); } \

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIDIRECTTASKDISPATCHER(_to) \
  NS_IMETHOD DispatchDirectTask(already_AddRefed<nsIRunnable> event) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DispatchDirectTask(event); } \
  NS_IMETHOD DrainDirectTasks(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DrainDirectTasks(); } \
  NS_IMETHOD HaveDirectTasks(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HaveDirectTasks(_retval); } \


#endif /* __gen_nsIDirectTaskDispatcher_h__ */

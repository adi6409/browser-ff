#include "ConsoleBinding.h"
#include "EventTargetBinding.h"
#include "URLBinding.h"
#include "URLSearchParamsBinding.h"
#include "WorkerDebuggerGlobalScopeBinding.h"

namespace mozilla {
namespace dom {
bool
RegisterWorkerDebuggerBindings(JSContext* aCx, JS::Handle<JSObject*> aObj)
{
  if (ConsoleInstance_Binding::ConstructorEnabled(aCx, aObj) && !ConsoleInstance_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (!EventTarget_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (!URL_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (!URLSearchParams_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (!WorkerDebuggerGlobalScope_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  if (!console_Binding::GetConstructorObject(aCx)) {
    return false;
  }

  return true;
}

} // namespace dom
} // namespace mozilla


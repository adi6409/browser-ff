// |reftest| shell-option(--enable-top-level-await) skip-if(!xulRuntime.shell) module -- requires shell-options
// This file was procedurally generated from the following sources:
// - src/top-level-await/await-expr-literal-string.case
// - src/top-level-await/syntax/void.template
/*---
description: AwaitExpression StringLiteral (Valid syntax for top level await in an UnaryExpression (void).)
esid: prod-AwaitExpression
features: [top-level-await]
flags: [generated, module]
info: |
    ModuleItem:
      StatementListItem[~Yield, +Await, ~Return]

    ...

    UnaryExpression[Yield, Await]
      void UnaryExpression[?Yield, ?Await]
      [+Await]AwaitExpression[?Yield]

    AwaitExpression[Yield]:
      await UnaryExpression[?Yield, +Await]

    ...


    PrimaryExpression[Yield, Await]:
      this
      IdentifierReference[?Yield, ?Await]
      Literal
      ArrayLiteral[?Yield, ?Await]
      ObjectLiteral[?Yield, ?Await]
      FunctionExpression
      ClassExpression[?Yield, ?Await]
      GeneratorExpression
      AsyncFunctionExpression
      AsyncGeneratorExpression
      RegularExpressionLiteral
      TemplateLiteral[?Yield, ?Await, ~Tagged]
      CoverParenthesizedExpressionAndArrowParameterList[?Yield, ?Await]

---*/


void await '';

reportCompare(0, 0);

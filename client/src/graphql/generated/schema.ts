// @ts-nocheck
import * as extensions from "../extensions";
import { lazyGetters } from "@gqless/utils";
import {
  ScalarNode,
  ObjectNode,
  FieldNode,
  ArrayNode,
  Arguments,
  ArgumentsField,
  EnumNode,
  InputNode,
  InputNodeField
} from "gqless";

export const schema = {
  get Boolean() {
    return new ScalarNode({
      name: "Boolean",
      extension: ((extensions as any) || {}).Boolean
    });
  },
  get __InputValue() {
    return new ObjectNode(
      {
        get name() {
          return new FieldNode(schema.String, undefined, false);
        },
        get description() {
          return new FieldNode(schema.String, undefined, true);
        },
        get type() {
          return new FieldNode(schema.__Type, undefined, false);
        },
        get defaultValue() {
          return new FieldNode(schema.String, undefined, true);
        }
      },
      {
        name: "__InputValue",
        extension: ((extensions as any) || {}).__InputValue
      }
    );
  },
  get String() {
    return new ScalarNode({
      name: "String",
      extension: ((extensions as any) || {}).String
    });
  },
  get __Field() {
    return new ObjectNode(
      {
        get name() {
          return new FieldNode(schema.String, undefined, false);
        },
        get description() {
          return new FieldNode(schema.String, undefined, true);
        },
        get args() {
          return new FieldNode(
            new ArrayNode(schema.__InputValue, false),
            undefined,
            false
          );
        },
        get type() {
          return new FieldNode(schema.__Type, undefined, false);
        },
        get isDeprecated() {
          return new FieldNode(schema.Boolean, undefined, false);
        },
        get deprecationReason() {
          return new FieldNode(schema.String, undefined, true);
        }
      },
      { name: "__Field", extension: ((extensions as any) || {}).__Field }
    );
  },
  get Mutation() {
    return new ObjectNode(
      {
        get createPost() {
          return new FieldNode(
            schema.Post,
            new Arguments(
              {
                get post() {
                  return new ArgumentsField(schema.NewPost, false);
                }
              },
              true
            ),
            false
          );
        },
        get createUser() {
          return new FieldNode(
            schema.User,
            new Arguments(
              {
                get user() {
                  return new ArgumentsField(schema.NewUser, false);
                }
              },
              true
            ),
            false
          );
        },
        get updateUser() {
          return new FieldNode(
            schema.User,
            new Arguments(
              {
                get user() {
                  return new ArgumentsField(schema.UpdateUser, false);
                }
              },
              true
            ),
            true
          );
        }
      },
      { name: "Mutation", extension: ((extensions as any) || {}).Mutation }
    );
  },
  get Post() {
    return new ObjectNode(
      {
        get id() {
          return new FieldNode(schema.Int, undefined, false);
        },
        get title() {
          return new FieldNode(schema.String, undefined, false);
        },
        get body() {
          return new FieldNode(schema.String, undefined, false);
        },
        get published() {
          return new FieldNode(schema.Boolean, undefined, false);
        }
      },
      { name: "Post", extension: ((extensions as any) || {}).Post }
    );
  },
  get __TypeKind() {
    return new EnumNode({ name: "__TypeKind" });
  },
  get __Type() {
    return new ObjectNode(
      {
        get name() {
          return new FieldNode(schema.String, undefined, true);
        },
        get description() {
          return new FieldNode(schema.String, undefined, true);
        },
        get kind() {
          return new FieldNode(schema.__TypeKind, undefined, false);
        },
        get fields() {
          return new FieldNode(
            new ArrayNode(schema.__Field, true),
            new Arguments({
              get includeDeprecated() {
                return new ArgumentsField(schema.Boolean, true);
              }
            }),
            true
          );
        },
        get ofType() {
          return new FieldNode(schema.__Type, undefined, true);
        },
        get inputFields() {
          return new FieldNode(
            new ArrayNode(schema.__InputValue, true),
            undefined,
            true
          );
        },
        get interfaces() {
          return new FieldNode(
            new ArrayNode(schema.__Type, true),
            undefined,
            true
          );
        },
        get possibleTypes() {
          return new FieldNode(
            new ArrayNode(schema.__Type, true),
            undefined,
            true
          );
        },
        get enumValues() {
          return new FieldNode(
            new ArrayNode(schema.__EnumValue, true),
            new Arguments({
              get includeDeprecated() {
                return new ArgumentsField(schema.Boolean, true);
              }
            }),
            true
          );
        }
      },
      { name: "__Type", extension: ((extensions as any) || {}).__Type }
    );
  },
  get __Schema() {
    return new ObjectNode(
      {
        get types() {
          return new FieldNode(
            new ArrayNode(schema.__Type, false),
            undefined,
            false
          );
        },
        get queryType() {
          return new FieldNode(schema.__Type, undefined, false);
        },
        get mutationType() {
          return new FieldNode(schema.__Type, undefined, true);
        },
        get subscriptionType() {
          return new FieldNode(schema.__Type, undefined, true);
        },
        get directives() {
          return new FieldNode(
            new ArrayNode(schema.__Directive, false),
            undefined,
            false
          );
        }
      },
      { name: "__Schema", extension: ((extensions as any) || {}).__Schema }
    );
  },
  get Role() {
    return new EnumNode({ name: "Role" });
  },
  get Int() {
    return new ScalarNode({
      name: "Int",
      extension: ((extensions as any) || {}).Int
    });
  },
  get Query() {
    return new ObjectNode(
      {
        get allPosts() {
          return new FieldNode(
            new ArrayNode(schema.Post, false),
            undefined,
            false
          );
        },
        get allUsers() {
          return new FieldNode(
            new ArrayNode(schema.User, false),
            undefined,
            false
          );
        }
      },
      { name: "Query", extension: ((extensions as any) || {}).Query }
    );
  },
  get NewPost() {
    return new InputNode(
      {
        get title() {
          return new InputNodeField(schema.String, false);
        },
        get body() {
          return new InputNodeField(schema.String, false);
        },
        get published() {
          return new InputNodeField(schema.Boolean, true);
        }
      },
      { name: "NewPost" }
    );
  },
  get NewUser() {
    return new InputNode(
      {
        get id() {
          return new InputNodeField(schema.Int, false);
        },
        get email() {
          return new InputNodeField(schema.String, false);
        },
        get name() {
          return new InputNodeField(schema.String, true);
        },
        get password() {
          return new InputNodeField(schema.String, false);
        }
      },
      { name: "NewUser" }
    );
  },
  get User() {
    return new ObjectNode(
      {
        get id() {
          return new FieldNode(schema.Int, undefined, false);
        },
        get email() {
          return new FieldNode(schema.String, undefined, false);
        },
        get name() {
          return new FieldNode(schema.String, undefined, false);
        },
        get password() {
          return new FieldNode(schema.String, undefined, false);
        },
        get role() {
          return new FieldNode(schema.Role, undefined, false);
        }
      },
      { name: "User", extension: ((extensions as any) || {}).User }
    );
  },
  get UpdateUser() {
    return new InputNode(
      {
        get id() {
          return new InputNodeField(schema.Int, false);
        },
        get email() {
          return new InputNodeField(schema.String, true);
        },
        get name() {
          return new InputNodeField(schema.String, true);
        },
        get password() {
          return new InputNodeField(schema.String, true);
        },
        get role() {
          return new InputNodeField(schema.Role, true);
        }
      },
      { name: "UpdateUser" }
    );
  },
  get __EnumValue() {
    return new ObjectNode(
      {
        get name() {
          return new FieldNode(schema.String, undefined, false);
        },
        get description() {
          return new FieldNode(schema.String, undefined, true);
        },
        get isDeprecated() {
          return new FieldNode(schema.Boolean, undefined, false);
        },
        get deprecationReason() {
          return new FieldNode(schema.String, undefined, true);
        }
      },
      {
        name: "__EnumValue",
        extension: ((extensions as any) || {}).__EnumValue
      }
    );
  },
  get __DirectiveLocation() {
    return new EnumNode({ name: "__DirectiveLocation" });
  },
  get __Directive() {
    return new ObjectNode(
      {
        get name() {
          return new FieldNode(schema.String, undefined, false);
        },
        get description() {
          return new FieldNode(schema.String, undefined, true);
        },
        get locations() {
          return new FieldNode(
            new ArrayNode(schema.__DirectiveLocation, false),
            undefined,
            false
          );
        },
        get args() {
          return new FieldNode(
            new ArrayNode(schema.__InputValue, false),
            undefined,
            false
          );
        },
        get onOperation() {
          return new FieldNode(schema.Boolean, undefined, false);
        },
        get onFragment() {
          return new FieldNode(schema.Boolean, undefined, false);
        },
        get onField() {
          return new FieldNode(schema.Boolean, undefined, false);
        }
      },
      {
        name: "__Directive",
        extension: ((extensions as any) || {}).__Directive
      }
    );
  }
};

lazyGetters(schema);

import * as extensions from "../extensions";
import {
  TypeData,
  ScalarType,
  FieldsType,
  FieldsTypeArg,
  EnumType
} from "gqless";

type Extension<TName extends string> = TName extends keyof typeof extensions
  ? typeof extensions[TName]
  : any;

/**
 * @name Boolean
 * @type SCALAR
 */
type t_Boolean<T extends boolean = boolean> = ScalarType<
  T,
  Extension<"Boolean">
>;

/**
 * @name __InputValue
 * @type OBJECT
 */
type t___InputValue = FieldsType<
  {
    __typename: t_String<"__InputValue">;
    name: t_String;
    description?: t_String | null;
    type: t___Type;
    defaultValue?: t_String | null;
  },
  Extension<"__InputValue">
>;

/**
 * @name String
 * @type SCALAR
 */
type t_String<T extends string = string> = ScalarType<T, Extension<"String">>;

/**
 * @name __Field
 * @type OBJECT
 */
type t___Field = FieldsType<
  {
    __typename: t_String<"__Field">;
    name: t_String;
    description?: t_String | null;
    args: t___InputValue[];
    type: t___Type;
    isDeprecated: t_Boolean;
    deprecationReason?: t_String | null;
  },
  Extension<"__Field">
>;

/**
 * @name Mutation
 * @type OBJECT
 */
type t_Mutation = FieldsType<
  {
    __typename: t_String<"Mutation">;
    createPost: FieldsTypeArg<{ post: NewPost }, t_Post>;
    createUser: FieldsTypeArg<{ user: NewUser }, t_User>;
    updateUser?: FieldsTypeArg<{ user: UpdateUser }, t_User | null>;
  },
  Extension<"Mutation">
>;

/**
 * @name Post
 * @type OBJECT
 */
type t_Post = FieldsType<
  {
    __typename: t_String<"Post">;
    id: t_Int;
    title: t_String;
    body: t_String;
    published: t_Boolean;
  },
  Extension<"Post">
>;

/**
 * @name __TypeKind
 * @type ENUM
 */
type t___TypeKind = EnumType<
  | "SCALAR"
  | "OBJECT"
  | "INTERFACE"
  | "UNION"
  | "ENUM"
  | "INPUT_OBJECT"
  | "LIST"
  | "NON_NULL"
>;

/**
 * @name __Type
 * @type OBJECT
 */
type t___Type = FieldsType<
  {
    __typename: t_String<"__Type">;
    name?: t_String | null;
    description?: t_String | null;
    kind: t___TypeKind;
    fields?: FieldsTypeArg<
      { includeDeprecated?: boolean | null },
      t___Field[] | null
    >;
    ofType?: t___Type | null;
    inputFields?: t___InputValue[] | null;
    interfaces?: t___Type[] | null;
    possibleTypes?: t___Type[] | null;
    enumValues?: FieldsTypeArg<
      { includeDeprecated?: boolean | null },
      t___EnumValue[] | null
    >;
  },
  Extension<"__Type">
>;

/**
 * @name __Schema
 * @type OBJECT
 */
type t___Schema = FieldsType<
  {
    __typename: t_String<"__Schema">;
    types: t___Type[];
    queryType: t___Type;
    mutationType?: t___Type | null;
    subscriptionType?: t___Type | null;
    directives: t___Directive[];
  },
  Extension<"__Schema">
>;

/**
 * @name Role
 * @type ENUM
 */
type t_Role = EnumType<"ADMIN" | "USER">;

/**
 * @name Int
 * @type SCALAR
 */
type t_Int<T extends number = number> = ScalarType<T, Extension<"Int">>;

/**
 * @name Query
 * @type OBJECT
 */
type t_Query = FieldsType<
  {
    __typename: t_String<"Query">;
    allPosts: t_Post[];
    allUsers: t_User[];
  },
  Extension<"Query">
>;

/**
 * @name NewPost
 * @type INPUT_OBJECT
 */
export type NewPost = {
  title: string;
  body: string;
  published?: boolean | null;
};

/**
 * @name NewUser
 * @type INPUT_OBJECT
 */
export type NewUser = {
  id: number;
  email: string;
  name?: string | null;
  password: string;
};

/**
 * @name User
 * @type OBJECT
 */
type t_User = FieldsType<
  {
    __typename: t_String<"User">;
    id: t_Int;
    email: t_String;
    name: t_String;
    password: t_String;
    role: t_Role;
  },
  Extension<"User">
>;

/**
 * @name UpdateUser
 * @type INPUT_OBJECT
 */
export type UpdateUser = {
  id: number;
  email?: string | null;
  name?: string | null;
  password?: string | null;
  role?: Role | null;
};

/**
 * @name __EnumValue
 * @type OBJECT
 */
type t___EnumValue = FieldsType<
  {
    __typename: t_String<"__EnumValue">;
    name: t_String;
    description?: t_String | null;
    isDeprecated: t_Boolean;
    deprecationReason?: t_String | null;
  },
  Extension<"__EnumValue">
>;

/**
 * @name __DirectiveLocation
 * @type ENUM
 */
type t___DirectiveLocation = EnumType<
  | "QUERY"
  | "MUTATION"
  | "SUBSCRIPTION"
  | "FIELD"
  | "FRAGMENT_DEFINITION"
  | "FRAGMENT_SPREAD"
  | "INLINE_FRAGMENT"
>;

/**
 * @name __Directive
 * @type OBJECT
 */
type t___Directive = FieldsType<
  {
    __typename: t_String<"__Directive">;
    name: t_String;
    description?: t_String | null;
    locations: t___DirectiveLocation[];
    args: t___InputValue[];

    /**
     * @deprecated Use the locations array instead
     */
    onOperation: t_Boolean;

    /**
     * @deprecated Use the locations array instead
     */
    onFragment: t_Boolean;

    /**
     * @deprecated Use the locations array instead
     */
    onField: t_Boolean;
  },
  Extension<"__Directive">
>;

/**
 * @name Boolean
 * @type SCALAR
 */
export type Boolean = TypeData<t_Boolean>;

/**
 * @name __InputValue
 * @type OBJECT
 */
export type __InputValue = TypeData<t___InputValue>;

/**
 * @name String
 * @type SCALAR
 */
export type String = TypeData<t_String>;

/**
 * @name __Field
 * @type OBJECT
 */
export type __Field = TypeData<t___Field>;

/**
 * @name Mutation
 * @type OBJECT
 */
export type Mutation = TypeData<t_Mutation>;

/**
 * @name Post
 * @type OBJECT
 */
export type Post = TypeData<t_Post>;

/**
 * @name __TypeKind
 * @type ENUM
 */
export enum __TypeKind {
  SCALAR = "SCALAR",
  OBJECT = "OBJECT",
  INTERFACE = "INTERFACE",
  UNION = "UNION",
  ENUM = "ENUM",
  INPUT_OBJECT = "INPUT_OBJECT",
  LIST = "LIST",
  NON_NULL = "NON_NULL"
}

/**
 * @name __Type
 * @type OBJECT
 */
export type __Type = TypeData<t___Type>;

/**
 * @name __Schema
 * @type OBJECT
 */
export type __Schema = TypeData<t___Schema>;

/**
 * @name Role
 * @type ENUM
 */
export enum Role {
  ADMIN = "ADMIN",
  USER = "USER"
}

/**
 * @name Int
 * @type SCALAR
 */
export type Int = TypeData<t_Int>;

/**
 * @name Query
 * @type OBJECT
 */
export type Query = TypeData<t_Query>;

/**
 * @name User
 * @type OBJECT
 */
export type User = TypeData<t_User>;

/**
 * @name __EnumValue
 * @type OBJECT
 */
export type __EnumValue = TypeData<t___EnumValue>;

/**
 * @name __DirectiveLocation
 * @type ENUM
 */
export enum __DirectiveLocation {
  QUERY = "QUERY",
  MUTATION = "MUTATION",
  SUBSCRIPTION = "SUBSCRIPTION",
  FIELD = "FIELD",
  FRAGMENT_DEFINITION = "FRAGMENT_DEFINITION",
  FRAGMENT_SPREAD = "FRAGMENT_SPREAD",
  INLINE_FRAGMENT = "INLINE_FRAGMENT"
}

/**
 * @name __Directive
 * @type OBJECT
 */
export type __Directive = TypeData<t___Directive>;

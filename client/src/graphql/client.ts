import { createUseMutation, createUseQuery } from "@gqless_hooks";
import { IS_BROWSER } from "@src/utils/constants";

import { Mutation, Query, schema } from "./generated";

const endpoint = IS_BROWSER
  ? "/api/graphql"
  : "http://localhost:8000/api/graphql";

export const { useQuery, prepareQuery } = createUseQuery<Query>({
  schema,
  endpoint,
});

export const useMutation = createUseMutation<Mutation>({
  schema,
  endpoint,
});

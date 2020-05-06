import { getArrayAccessorFields } from "gqless-hooks";
import { GetServerSideProps, NextPage } from "next";

import { prepareQuery } from "@src/graphql";

const IndexProps = prepareQuery({
  cacheId: "Query",
  query: schema => {
    return getArrayAccessorFields(schema.allUsers, "id", "email", "name");
  }
});

interface Props {
  users: typeof IndexProps.dataType;
}

export const getServerSideProps: GetServerSideProps<Props> = async () => {
  const posts = await IndexProps.prepare({
    checkCache: false
  });

  return {
    props: {
      users: posts
    }
  };
};

const IndexPage: NextPage<Props> = props => {
  return <div>{JSON.stringify(props.users)}</div>;
};

export default IndexPage;

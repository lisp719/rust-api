import { gql, useMutation, useQuery } from "@apollo/client";
import React from "react";

const GET_UPLOADS = gql`
  query GetUploads {
    uploads {
      id
      url
    }
  }
`;

const UPLOAD_FILES = gql`
  mutation ($files: [Upload!]!) {
    multipleUpload(files: $files) {
      id
      url
    }
  }
`;

function DisplayUploads() {
  const { loading, error, data } = useQuery(GET_UPLOADS);

  if (loading) return <p>Loading...</p>;
  if (error) return <p>Error : {error.message}</p>;

  return data.uploads.map(({ id, url }: { id: string; url: string }) => (
    <div key={id}>
      <p>{url}</p>
    </div>
  ));
}

function UploadFiles() {
  const [mutate] = useMutation(UPLOAD_FILES, {
    refetchQueries: [{ query: GET_UPLOADS }],
  });

  const onChange = ({
    target: { validity, files },
  }: React.ChangeEvent<HTMLInputElement>) => {
    if (validity.valid) {
      mutate({ variables: { files } });
    }
  };

  return <input type="file" multiple required onChange={onChange} />;
}

function App() {
  return (
    <div>
      <h3>upload</h3>
      <UploadFiles />
      <hr />
      <DisplayUploads />
    </div>
  );
}

export default App;

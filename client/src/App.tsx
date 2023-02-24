import { gql, useQuery } from "@apollo/client";

const GET_UPLOADS = gql`
  query GetUploads {
    uploads {
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

function App() {
  return (
    <div>
      <h3>upload</h3>
      <DisplayUploads />
    </div>
  );
}

export default App;

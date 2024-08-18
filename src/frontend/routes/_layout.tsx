import { PageProps } from "$fresh/server.ts";
import Footer from "./_footer.tsx";
import Head from "./_head.tsx";
import Header from "./_header.tsx";

export default function Layout({ Component }: PageProps) {
  return (
    <html lang="en">
      <Head />
      <body class="flex flex-col min-h-dvh">
        <Header />
        <div class="container my-4 mx-4 flex-1">
          <Component />
        </div>
        <Footer />
      </body>
    </html>
  );
}

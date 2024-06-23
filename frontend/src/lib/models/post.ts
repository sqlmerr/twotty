export default interface Post {
  id: string;
  text: string;
  authorId: string;
  edited: boolean;
  createdAt: Date;
}

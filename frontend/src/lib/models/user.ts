export default interface User {
  id: string;
  username: string;
  avatar: null | string;
  about: string;
  followers?: number;
  followings?: number;
}

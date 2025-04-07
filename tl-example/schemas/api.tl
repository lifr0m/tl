type Message id:int32 text:string? photos:[bytes] sent_at:time = Message
type User id:int64 verified:bool rating:float = User
type UserEmpty id:int64 = User

error InvalidUserId user_id:int64
error TooLongText text:string max_length:int32

func get_users user_ids:[int64] = [User]
func send_message user_id:int64 text:string? photos:[bytes] = Message

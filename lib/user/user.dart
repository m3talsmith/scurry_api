import 'package:cryptography/cryptography.dart';
import 'package:scurry_api/provider/database/database.dart';

class User {
  String? uid;
  String? name;
  String? _password;

  User({this.uid, this.name, String? password}) {
    password = password ?? '';
  }

  User.fromJson(Map<String, dynamic> json) {
    uid = json['uid'] as String?;
    name = json['name'] as String?;
    password = json['password'] as String? ?? '';
  }

  Map<String, dynamic> toJson() => {
        'uid': uid,
        'name': name,
      };

  static String hashPassword(String p) => Sha512().hash(p.codeUnits).toString();

  String get password => _password!;
  set password(String p) => _password = User.hashPassword(p);

  Future<void> create() async {
    final client = Database.client;
    final data = await client
        .from('users')
        .insert(toJson())
        .select<Map<String, dynamic>>('uid, name');
    uid = data['uid'] as String?;
  }
}

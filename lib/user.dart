import 'package:cryptography/cryptography.dart';

class User {
    String? uid;
    late String name;
    late String _password;

    User({this.uid, required this.name, String? password}) {
        password = password ?? "";
    }

    static String hashPassword(String p) async => await Sha512().hash(p);

    String get password => _password;
    set password(String p) => _password = User.hashPassword(p);
}